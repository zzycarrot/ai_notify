use ai_notify::{UserContext, NotificationFilter, SystemNotification, NotificationListener};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

/// 通知决策引擎
struct NotificationEngine {
    filter: NotificationFilter,
    listener: NotificationListener,
}

impl NotificationEngine {
    fn new() -> Self {
        Self {
            filter: NotificationFilter::new(),
            listener: NotificationListener::new(),
        }
    }

    /// 处理单个通知
    fn process_notification(
        &self,
        notification: &SystemNotification,
        context: &UserContext,
    ) -> bool {
        let filter_result = self.filter.filter(&notification.body, context);

        info!(
            notification = %notification.body,
            app = %context.app_name,
            activity = %context.analyze_activity(),
            should_show = filter_result.should_show,
            reason = %filter_result.reason,
            confidence = filter_result.confidence,
            "Notification processed"
        );

        filter_result.should_show
    }

    /// 运行通知处理循环
    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting notification engine...");

        let receiver = self.listener.start()?;

        loop {
            // 获取当前用户上下文
            let context = match UserContext::current() {
                Some(ctx) => ctx,
                None => {
                    warn!("Could not get current user context");
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            };

            info!(
                app = %context.app_name,
                window_title = %context.window_title,
                "Current context"
            );

            // 检查是否有新通知（非阻塞检查）
            match receiver.try_recv() {
                Ok(notification) => {
                    info!("New notification received: {}", notification.title);
                    let should_show = self.process_notification(&notification, &context);

                    if should_show {
                        info!("✓ Showing notification: {}", notification.title);
                    } else {
                        info!("✗ Filtering out notification: {}", notification.title);
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    // No new notifications
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    warn!("Notification receiver disconnected");
                    break;
                }
            }

            // 定期检查
            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("AI Notification Filter starting...");

    // 创建并运行引擎
    let mut engine = NotificationEngine::new();
    engine.run().await?;

    Ok(())
}