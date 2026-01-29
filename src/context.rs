use active_win_pos_rs::get_active_window;
use serde_json::json;
use std::fmt;

/// 代表用户当前的活动上下文
#[derive(Clone, Debug)]
pub struct UserContext {
    /// 应用名称 (e.g., "Google Chrome", "Safari")
    pub app_name: String,
    /// 窗口标题 (e.g., "GitHub Copilot - VS Code")
    pub window_title: String,
}

/// 活动类型分类
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityType {
    /// 工作相关
    Work,
    /// 学习相关
    Learning,
    /// 娱乐相关
    Entertainment,
    /// 未知/其他
    Unknown,
}

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Work => write!(f, "Work"),
            Self::Learning => write!(f, "Learning"),
            Self::Entertainment => write!(f, "Entertainment"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl UserContext {
    /// 创建新的用户上下文 (用于测试)
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            window_title: String::new(),
        }
    }

    /// 获取当前用户上下文
    pub fn current() -> Option<Self> {
        match get_active_window() {
            Ok(window) => Some(UserContext {
                app_name: window.app_name,
                window_title: window.title,
            }),
            Err(_) => None,
        }
    }

    /// 分析当前活动类型
    pub fn analyze_activity(&self) -> ActivityType {
        let app_lower = self.app_name.to_lowercase();
        let title_lower = self.window_title.to_lowercase();

        // 工作关键词
        if Self::contains_any_keyword(&title_lower, &[
            "vscode", "intellij", "xcode", "visual studio", "github", "gitlab", "jira",
            "slack", "teams", "confluence", "notion", "work", "office", "excel", "word",
            "powerpoint", "sheets", "docs", "drive", "devops", "jenkins", "docker",
        ]) || Self::contains_any_keyword(&app_lower, &["code", "idea", "xcode", "slack", "teams"])
        {
            return ActivityType::Work;
        }

        // 学习关键词
        if Self::contains_any_keyword(&title_lower, &[
            "docs", "documentation", "tutorial", "course", "learn", "book", "pdf",
            "wikipedia", "stack overflow", "github", "dev.to", "medium", "blog",
            "educative", "coursera", "udemy", "duolingo", "khan academy",
        ]) || Self::contains_any_keyword(&app_lower, &["chrome", "firefox", "safari"])
        {
            // 如果是浏览器打开文档相关页面
            if title_lower.contains("doc") || title_lower.contains("learn") || title_lower.contains("tutorial")
            {
                return ActivityType::Learning;
            }
        }

        // 娱乐关键词
        if Self::contains_any_keyword(&title_lower, &[
            "bilibili", "youtube", "netflix", "hulu", "prime video", "twitch",
            "lol", "dota", "steam", "game", "discord", "qq", "wechat", "telegram",
            "twitter", "facebook", "instagram", "tiktok", "reddit", "4chan",
        ]) {
            return ActivityType::Entertainment;
        }

        ActivityType::Unknown
    }

    /// 辅助函数：检查是否包含任何关键词
    fn contains_any_keyword(text: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| text.contains(kw))
    }

    /// 转为JSON格式
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "app_name": self.app_name,
            "window_title": self.window_title,
            "activity_type": self.analyze_activity().to_string(),
        })
    }
}