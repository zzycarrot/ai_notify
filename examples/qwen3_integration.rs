/// Qwen3-0.6B 集成示例程序
/// 演示如何使用混合通知处理器与 LLM 推理

use ai_notify::{
    context::UserContext,
    engine::{HybridNotificationHandler, NotificationHandlerConfig},
    llm::LLMConfig,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Qwen3-0.6B 通知筛选示例程序启动");

    // 示例 1: 基础配置（关键词过滤）
    info!("\n=== 示例 1: 关键词过滤 ===");
    keyword_filtering_example().await?;

    // 示例 2: LLM 配置（需要 API 密钥）
    info!("\n=== 示例 2: LLM 集成（需要配置）===");
    llm_integration_example().await?;

    // 示例 3: 缓存演示
    info!("\n=== 示例 3: 缓存机制 ===");
    cache_example().await?;

    info!("\n✅ 所有示例执行完成!");
    Ok(())
}

/// 示例 1: 关键词过滤（默认配置）
async fn keyword_filtering_example() -> anyhow::Result<()> {
    let handler = HybridNotificationHandler::with_default_config();
    let mut context = UserContext::new();
    context.app_name = "VSCode".to_string();
    context.window_title = "project/main.rs".to_string();

    let notifications = vec![
        ("Code Review", "Your PR has been approved", "GitHub", true),
        ("Limited Offer", "limited offer today only!", "Store", false),
        ("Critical Alert", "System performance critical", "Monitor", true),
    ];

    for (title, body, app, expected) in notifications {
        let result = handler
            .process_notification(title, body, app, &context)
            .await?;

        let emoji = if result.should_show { "✅" } else { "❌" };
        info!(
            "{} [{}] {} - {} | 方法: {}, 置信度: {:.2}",
            emoji,
            app,
            title,
            body,
            result.method,
            result.confidence
        );

        assert_eq!(
            result.should_show, expected,
            "Unexpected result for {}",
            title
        );
    }

    Ok(())
}

/// 示例 2: LLM 集成（需要配置）
async fn llm_integration_example() -> anyhow::Result<()> {
    // 创建 LLM 配置
    let llm_config = LLMConfig {
        model_id: "Qwen/Qwen3-0.6B".to_string(),
        enable_thinking: false,
        max_tokens: 512,
        temperature: 0.7,
        top_p: 0.8,
        local_mode: false,
    };

    // 创建处理器配置
    let handler_config = NotificationHandlerConfig {
        enable_llm: false,  // 设为 false 以避免 API 调用（除非配置了 HF_TOKEN）
        llm_confidence_threshold: 0.7,
        enable_cache: true,
        llm_config,
    };

    let handler = HybridNotificationHandler::new(handler_config);

    info!("LLM 配置信息:");
    info!("  模型: Qwen/Qwen3-0.6B");
    info!("  最大令牌: 512");
    info!("  温度: 0.7");
    info!("  Top-P: 0.8");

    // 演示如何启用 LLM
    info!("\n启用 LLM 的方法:");
    info!("1. 设置 HF_TOKEN 环境变量");
    info!("   export HF_TOKEN=hf_xxxxxxxxxxxxxxxxxxxx");
    info!("2. 设置 enable_llm=true");
    info!("3. 或使用本地模型 (local_mode=true)");

    let context = UserContext::new();
    let result = handler
        .process_notification(
            "Meeting Scheduled",
            "Team sync at 3 PM",
            "Calendar",
            &context,
        )
        .await?;

    info!("处理结果: {}", if result.should_show { "显示" } else { "隐藏" });
    info!("原因: {}", result.reason);

    Ok(())
}

/// 示例 3: 缓存演示
async fn cache_example() -> anyhow::Result<()> {
    use ai_notify::llm::LLMCache;

    let cache = LLMCache::new();

    info!("缓存演示:");
    info!("初始缓存大小: {}", cache.size().await);

    // 生成缓存键
    let key1 = LLMCache::make_key("Title1", "Body1", "App1");
    let key2 = LLMCache::make_key("Title2", "Body2", "App2");

    // 模拟缓存数据
    let analysis = ai_notify::LLMAnalysis {
        priority: 8,
        category: "work".to_string(),
        action: "show".to_string(),
        confidence: 0.95,
        reasoning: "Work-related notification".to_string(),
    };

    cache.set(key1.clone(), analysis.clone()).await;
    cache.set(key2.clone(), analysis).await;

    info!("添加 2 条缓存后的大小: {}", cache.size().await);

    // 检索缓存
    if let Some(cached) = cache.get(&key1).await {
        info!(
            "检索到缓存 - 优先级: {}, 类别: {}, 行动: {}",
            cached.priority, cached.category, cached.action
        );
    }

    // 清空缓存
    cache.clear().await;
    info!("清空缓存后的大小: {}", cache.size().await);

    Ok(())
}
