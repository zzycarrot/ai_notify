#[cfg(test)]
mod tests {
    use ai_notify::{
        context::UserContext,
        engine::{HybridNotificationHandler, NotificationHandlerConfig},
        llm::LLMConfig,
    };

    #[tokio::test]
    async fn test_hybrid_handler_keyword_priority() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        // 测试通知处理 - 关键是验证结构而不是具体数值
        let result = handler
            .process_notification(
                "System Alert",
                "Critical system failure detected",
                "SystemMonitor",
                &context,
            )
            .await
            .unwrap();

        // 应该显示紧急通知
        assert!(result.should_show);
        // 方法应该是某种过滤方法
        assert!(result.method.contains("keyword"));
        // 置信度应该是有效范围
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[tokio::test]
    async fn test_spam_filtering() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        // 测试垃圾消息总是被过滤 - 需要包含确实的垃圾关键词
        let result = handler
            .process_notification(
                "Limited Offer",
                "Click here for amazing limited offer!",
                "AdNetwork",
                &context,
            )
            .await
            .unwrap();

        // "limited offer" 包含 "limited offer" 关键词
        assert!(!result.should_show);
        assert!(result.reason.to_lowercase().contains("spam"));
    }

    #[tokio::test]
    async fn test_work_context_notification() {
        let handler = HybridNotificationHandler::with_default_config();
        let mut context = UserContext::new();
        context.app_name = "Code Editor".to_string();
        context.window_title = "project/main.rs - GitHub".to_string();

        // 工作相关的通知应该显示
        let result = handler
            .process_notification(
                "Code Review",
                "Your pull request has been reviewed",
                "GitHub",
                &context,
            )
            .await
            .unwrap();

        assert!(result.should_show);
    }

    #[tokio::test]
    async fn test_llm_config_creation() {
        let config = LLMConfig {
            model_id: "Qwen/Qwen3-0.6B".to_string(),
            enable_thinking: true,
            max_tokens: 1024,
            temperature: 0.6,
            top_p: 0.95,
            local_mode: true,
        };

        assert_eq!(config.model_id, "Qwen/Qwen3-0.6B");
        assert!(config.enable_thinking);
        assert!(config.local_mode);
    }

    #[tokio::test]
    async fn test_handler_config_with_llm() {
        let config = NotificationHandlerConfig {
            enable_llm: true,
            llm_confidence_threshold: 0.75,
            enable_cache: true,
            llm_config: LLMConfig {
                model_id: "Qwen/Qwen3-0.6B".to_string(),
                enable_thinking: false,
                max_tokens: 512,
                temperature: 0.7,
                top_p: 0.8,
                local_mode: false,
            },
        };

        assert!(config.enable_llm);
        assert!(config.enable_cache);
        assert_eq!(config.llm_config.model_id, "Qwen/Qwen3-0.6B");
    }

    #[tokio::test]
    async fn test_multiple_notifications_processing() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        let notifications = vec![
            ("System Alert", "CPU usage critical", "Monitor", true),
            ("Promotional", "limited offer today only", "Store", false),
            ("Message", "Hey, how are you?", "Chat", true),
        ];

        for (title, body, app, expected_show) in notifications {
            let result = handler
                .process_notification(title, body, app, &context)
                .await
                .unwrap();

            assert_eq!(
                result.should_show, expected_show,
                "Failed for notification: {} from {}",
                title, app
            );
        }
    }

    #[test]
    fn test_llm_config_defaults() {
        let config = LLMConfig::default();
        assert_eq!(config.model_id, "Qwen/Qwen3-0.6B");
        assert!(!config.enable_thinking); // 默认禁用思考模式以提高速度
        assert_eq!(config.max_tokens, 512);
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.top_p, 0.8);
    }

    #[tokio::test]
    async fn test_learning_context_entertainment_filtering() {
        let handler = HybridNotificationHandler::with_default_config();
        let mut context = UserContext::new();
        context.app_name = "Code Editor".to_string();
        context.window_title = "Learning Python basics".to_string();

        // 娱乐相关的通知在学习上下文中应该被过滤
        let result = handler
            .process_notification(
                "Video Ready",
                "Your game video is ready to play",
                "Entertainment",
                &context,
            )
            .await
            .unwrap();

        // 根据关键词"game"应该被过滤
        assert!(!result.should_show);
    }

    #[tokio::test]
    async fn test_processing_result_structure() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        let result = handler
            .process_notification(
                "Test Title",
                "Test Body",
                "TestApp",
                &context,
            )
            .await
            .unwrap();

        // 验证结果结构
        assert!(!result.method.is_empty());
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        assert!(!result.reason.is_empty());
    }
}
