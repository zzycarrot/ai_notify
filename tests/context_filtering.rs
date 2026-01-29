// 示例：测试不同上下文中的通知过滤
// 运行方式：cargo test --lib test_context_filtering -- --nocapture

#[cfg(test)]
mod tests {
    use ai_notify::{UserContext, NotificationFilter};

    #[test]
    fn test_work_context_filtering() {
        let filter = NotificationFilter::new();
        
        let work_context = UserContext {
            app_name: "VSCode".to_string(),
            window_title: "main.rs - ai_notify".to_string(),
        };
        
        // 工作消息应该显示
        let work_notification = "Meeting: Sprint Planning at 2PM";
        let result = filter.filter(work_notification, &work_context);
        assert!(result.should_show, "Work notification should be shown in work context");
        
        println!("✓ Work context test passed: {}", result.reason);
    }

    #[test]
    fn test_learning_context_filtering() {
        let filter = NotificationFilter::new();
        
        let learning_context = UserContext {
            app_name: "Safari".to_string(),
            window_title: "Rust Programming Language - Documentation".to_string(),
        };
        
        // 学习相关内容显示
        let learning_notification = "Your Stack Overflow answer was accepted!";
        let result = filter.filter(learning_notification, &learning_context);
        assert!(result.should_show, "Learning notification should be shown in learning context");
        
        // 娱乐内容过滤
        let distraction_notification = "Check out this gaming video";
        let result = filter.filter(distraction_notification, &learning_context);
        assert!(!result.should_show, "Entertainment should be filtered in learning context");
        
        println!("✓ Learning context test passed");
    }

    #[test]
    fn test_entertainment_context_filtering() {
        let filter = NotificationFilter::new();
        
        let entertainment_context = UserContext {
            app_name: "Chrome".to_string(),
            window_title: "Bilibili - Latest Gaming Videos".to_string(),
        };
        
        // 娱乐上下文显示所有非垃圾消息
        let normal_notification = "Your friend sent you a message";
        let result = filter.filter(normal_notification, &entertainment_context);
        assert!(result.should_show, "Normal notifications should be shown in entertainment context");
        
        // 垃圾消息仍然被过滤
        let spam_notification = "CLICK HERE for limited offer!";
        let result = filter.filter(spam_notification, &entertainment_context);
        assert!(!result.should_show, "Spam should be filtered even in entertainment context");
        
        println!("✓ Entertainment context test passed");
    }

    #[test]
    fn test_urgent_notifications_always_shown() {
        let filter = NotificationFilter::new();
        
        let any_context = UserContext {
            app_name: "Chrome".to_string(),
            window_title: "Bilibili - Videos".to_string(),
        };
        
        // 紧急消息在任何上下文中都应显示
        let urgent_notifications = vec![
            "ALERT: Security breach detected",
            "CRITICAL: System down",
            "ERROR: Database connection failed",
        ];
        
        for notification in urgent_notifications {
            let result = filter.filter(notification, &any_context);
            assert!(result.should_show, "Urgent notification '{}' should always be shown", notification);
        }
        
        println!("✓ Urgent notifications test passed");
    }

    #[test]
    fn test_activity_classification() {
        let work_context = UserContext {
            app_name: "VSCode".to_string(),
            window_title: "project.rs".to_string(),
        };
        let activity = work_context.analyze_activity();
        assert_eq!(activity.to_string(), "Work");
        
        let learning_context = UserContext {
            app_name: "Firefox".to_string(),
            window_title: "Stack Overflow - rust documentation".to_string(),
        };
        let activity = learning_context.analyze_activity();
        assert_eq!(activity.to_string(), "Learning");
        
        let entertainment_context = UserContext {
            app_name: "Chrome".to_string(),
            window_title: "YouTube - LOL Highlights".to_string(),
        };
        let activity = entertainment_context.analyze_activity();
        assert_eq!(activity.to_string(), "Entertainment");
        
        println!("✓ Activity classification test passed");
    }
}