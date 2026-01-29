use crate::context::{UserContext, ActivityType};

/// 通知过滤结果
#[derive(Clone, Debug)]
pub struct FilterResult {
    /// 是否应该显示通知
    pub should_show: bool,
    /// 过滤原因/解释
    pub reason: String,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f32,
}

/// 通知过滤器
pub struct NotificationFilter {
    /// 工作类型消息关键词
    work_keywords: Vec<String>,
    /// 紧急消息关键词 (总是显示)
    urgent_keywords: Vec<String>,
    /// 垃圾消息关键词 (总是过滤)
    spam_keywords: Vec<String>,
}

impl NotificationFilter {
    pub fn new() -> Self {
        Self {
            work_keywords: vec![
                "meeting".to_string(),
                "deadline".to_string(),
                "review".to_string(),
                "deploy".to_string(),
                "bug".to_string(),
                "issue".to_string(),
                "pull request".to_string(),
                "urgent".to_string(),
            ],
            urgent_keywords: vec![
                "alert".to_string(),
                "critical".to_string(),
                "emergency".to_string(),
                "security".to_string(),
                "error".to_string(),
                "system down".to_string(),
            ],
            spam_keywords: vec![
                "ad".to_string(),
                "advertisement".to_string(),
                "click here".to_string(),
                "spam".to_string(),
                "promotion".to_string(),
                "limited offer".to_string(),
            ],
        }
    }

    /// 根据上下文和通知内容过滤
    pub fn filter(&self, notification_text: &str, context: &UserContext) -> FilterResult {
        let activity_type = context.analyze_activity();

        // 紧急消息总是显示
        if self.contains_keywords(notification_text, &self.urgent_keywords) {
            return FilterResult {
                should_show: true,
                reason: "Critical/Urgent notification".to_string(),
                confidence: 0.95,
            };
        }

        // 垃圾消息总是过滤
        if self.contains_keywords(notification_text, &self.spam_keywords) {
            return FilterResult {
                should_show: false,
                reason: "Detected spam keywords".to_string(),
                confidence: 0.9,
            };
        }

        // 根据活动类型过滤
        match activity_type {
            ActivityType::Work => self.filter_work_context(notification_text),
            ActivityType::Learning => self.filter_learning_context(notification_text),
            ActivityType::Entertainment => self.filter_entertainment_context(notification_text),
            ActivityType::Unknown => self.filter_unknown_context(notification_text),
        }
    }

    /// 工作上下文过滤
    fn filter_work_context(&self, notification_text: &str) -> FilterResult {
        let lower = notification_text.to_lowercase();

        // 工作相关通知显示
        if self.contains_keywords(&lower, &self.work_keywords) {
            return FilterResult {
                should_show: true,
                reason: "Work-related notification in work context".to_string(),
                confidence: 0.85,
            };
        }

        // 个人消息可能较低优先级
        if Self::contains_keywords_str(&lower, &["hello", "hi", "hey", "thanks", "thanks for"]) {
            return FilterResult {
                should_show: true,
                reason: "Personal message in work context".to_string(),
                confidence: 0.7,
            };
        }

        // 默认显示工作消息
        FilterResult {
            should_show: true,
            reason: "Default show in work context".to_string(),
            confidence: 0.6,
        }
    }

    /// 学习上下文过滤
    fn filter_learning_context(&self, notification_text: &str) -> FilterResult {
        let lower = notification_text.to_lowercase();

        // 工作相关消息显示（可能是deadline提醒等）
        if self.contains_keywords(&lower, &self.work_keywords) {
            return FilterResult {
                should_show: true,
                reason: "Work notification during learning".to_string(),
                confidence: 0.8,
            };
        }

        // 娱乐/分心消息过滤
        if Self::contains_keywords_str(&lower, &["game", "video", "watch", "stream", "play"])
        {
            return FilterResult {
                should_show: false,
                reason: "Entertainment distraction during learning".to_string(),
                confidence: 0.75,
            };
        }

        FilterResult {
            should_show: true,
            reason: "Learning context - allowing notification".to_string(),
            confidence: 0.6,
        }
    }

    /// 娱乐上下文过滤
    fn filter_entertainment_context(&self, _notification_text: &str) -> FilterResult {
        // 娱乐时显示所有非垃圾消息
        FilterResult {
            should_show: true,
            reason: "Entertainment context - showing all notifications".to_string(),
            confidence: 0.5,
        }
    }

    /// 未知上下文过滤
    fn filter_unknown_context(&self, notification_text: &str) -> FilterResult {
        let lower = notification_text.to_lowercase();

        // 保守策略：紧急的显示，其他的默认显示
        if self.contains_keywords(&lower, &self.work_keywords) {
            return FilterResult {
                should_show: true,
                reason: "Work-related in unknown context".to_string(),
                confidence: 0.65,
            };
        }

        FilterResult {
            should_show: true,
            reason: "Unknown context - default show".to_string(),
            confidence: 0.5,
        }
    }

    /// 检查是否包含关键词
    fn contains_keywords(&self, text: &str, keywords: &[String]) -> bool {
        keywords.iter().any(|kw| text.contains(kw.as_str()))
    }

    /// 检查是否包含关键词 (string slice version)
    fn contains_keywords_str(text: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| text.contains(kw))
    }
}

impl Default for NotificationFilter {
    fn default() -> Self {
        Self::new()
    }
}