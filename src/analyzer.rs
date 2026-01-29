/// 通知内容分析
#[derive(Clone, Debug)]
pub struct NotificationAnalysis {
    /// 原始文本
    pub text: String,
    /// 提取的标题
    pub title: Option<String>,
    /// 提取的主体
    pub body: Option<String>,
    /// 应用名称
    pub app_name: Option<String>,
}

impl NotificationAnalysis {
    pub fn new(text: String) -> Self {
        Self {
            text,
            title: None,
            body: None,
            app_name: None,
        }
    }

    /// 从通知对象分析内容
    pub fn analyze(&mut self) {
        // 提取标题 (通常是第一行或特定格式)
        let lines: Vec<&str> = self.text.lines().collect();
        if !lines.is_empty() {
            self.title = Some(lines[0].to_string());
        }

        // 提取主体 (剩余内容)
        if lines.len() > 1 {
            self.body = Some(lines[1..].join("\n"));
        } else {
            self.body = Some(self.text.clone());
        }
    }

    /// 获取完整通知内容
    pub fn full_content(&self) -> String {
        format!(
            "{}{}",
            self.title.as_ref().map(|t| format!("{}\n", t)).unwrap_or_default(),
            self.body.as_ref().map(|b| b.clone()).unwrap_or_default()
        )
    }
}