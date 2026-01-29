use objc2::rc::autoreleasepool;
use std::sync::mpsc::{channel, Receiver};

/// 代表一条系统通知
#[derive(Clone, Debug)]
pub struct SystemNotification {
    pub title: String,
    pub body: String,
    pub app_name: String,
    pub timestamp: std::time::SystemTime,
}

/// macOS通知监听器
pub struct NotificationListener {
    receiver: Option<Receiver<SystemNotification>>,
}

impl NotificationListener {
    pub fn new() -> Self {
        Self { receiver: None }
    }

    /// 启动监听
    pub fn start(&mut self) -> Result<Receiver<SystemNotification>, anyhow::Error> {
        let (tx, rx) = channel();

        // 在独立线程中运行通知中心监听
        std::thread::spawn(move || {
            autoreleasepool(|_| {
                // 为演示目的，这是一个简化的实现
                // 实际生产环境需要更复杂的设置
                tracing::info!("Notification listener initialized");

                // 发送演示通知用于测试
                let demo_notification = SystemNotification {
                    title: "Demo Notification".to_string(),
                    body: "This is a test notification".to_string(),
                    app_name: "System".to_string(),
                    timestamp: std::time::SystemTime::now(),
                };

                let _ = tx.send(demo_notification);
            });
        });

        Ok(rx)
    }

    /// 获取接收器
    pub fn get_receiver(&self) -> Option<&Receiver<SystemNotification>> {
        self.receiver.as_ref()
    }

    /// 获取可变接收器
    pub fn get_receiver_mut(&mut self) -> Option<&mut Receiver<SystemNotification>> {
        self.receiver.as_mut()
    }
}

impl Default for NotificationListener {
    fn default() -> Self {
        Self::new()
    }
}