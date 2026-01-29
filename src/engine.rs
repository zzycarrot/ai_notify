/// 混合通知处理引擎
/// 结合传统关键词过滤和 LLM 推理
/// 
/// 架构:
/// 1. 快速路径: 关键词规则 (本地, <10ms)
/// 2. 标准路径: LLM 推理 (可配置, 需要 API 或本地模型)
/// 3. 缓存层: 避免重复推理

use crate::context::UserContext;
use crate::filter::{NotificationFilter, FilterResult};
use crate::llm::{LLMAnalysis, LLMCache, LLMClient, LLMConfig};
use anyhow::Result;
use tracing::{debug, warn};

/// 通知处理配置
#[derive(Clone, Debug)]
pub struct NotificationHandlerConfig {
    /// 是否启用 LLM 推理
    pub enable_llm: bool,
    /// LLM 置信度阈值 (低于此值时使用关键词过滤)
    pub llm_confidence_threshold: f32,
    /// 启用缓存
    pub enable_cache: bool,
    /// LLM 配置
    pub llm_config: LLMConfig,
}

impl Default for NotificationHandlerConfig {
    fn default() -> Self {
        Self {
            enable_llm: false, // 默认关闭 LLM，等待配置
            llm_confidence_threshold: 0.7,
            enable_cache: true,
            llm_config: LLMConfig::default(),
        }
    }
}

/// 混合通知处理器
pub struct HybridNotificationHandler {
    config: NotificationHandlerConfig,
    keyword_filter: NotificationFilter,
    llm_client: Option<LLMClient>,
    cache: LLMCache,
}

impl HybridNotificationHandler {
    pub fn new(config: NotificationHandlerConfig) -> Self {
        let llm_client = if config.enable_llm {
            Some(LLMClient::new(config.llm_config.clone()))
        } else {
            None
        };

        Self {
            config,
            keyword_filter: NotificationFilter::new(),
            llm_client,
            cache: LLMCache::new(),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(NotificationHandlerConfig::default())
    }

    /// 处理通知 - 返回是否应该显示
    pub async fn process_notification(
        &self,
        title: &str,
        body: &str,
        app_name: &str,
        context: &UserContext,
    ) -> Result<ProcessingResult> {
        let notification_text = format!("{} {}", title, body);

        // 第一步: 快速关键词过滤
        let keyword_result = self.keyword_filter.filter(&notification_text, context);
        debug!(
            "Keyword filter result: should_show={}, reason={}",
            keyword_result.should_show, keyword_result.reason
        );

        // 如果关键词过滤有高置信度，直接返回
        if keyword_result.confidence > 0.85 {
            return Ok(ProcessingResult {
                should_show: keyword_result.should_show,
                method: "keyword_filter".to_string(),
                confidence: keyword_result.confidence,
                reason: keyword_result.reason,
                llm_analysis: None,
            });
        }

        // 第二步: 如果启用 LLM 且置信度不够高，使用 LLM 推理
        if self.config.enable_llm && self.llm_client.is_some() {
            let activity = context.analyze_activity();
            let cache_key = LLMCache::make_key(title, body, app_name);

            // 检查缓存
            if self.config.enable_cache {
                if let Some(cached) = self.cache.get(&cache_key).await {
                    debug!("LLM result from cache");
                    return Ok(self.merge_results(
                        keyword_result,
                        cached,
                        "llm_cache".to_string(),
                    ));
                }
            }

            // 调用 LLM
            if let Ok(llm_result) = self
                .llm_client
                .as_ref()
                .unwrap()
                .analyze_notification(title, body, app_name, &format!("{:?}", activity))
                .await
            {
                // 缓存结果
                if self.config.enable_cache {
                    self.cache.set(cache_key, llm_result.clone()).await;
                }

                debug!(
                    "LLM analysis: priority={}, category={}, action={}",
                    llm_result.priority, llm_result.category, llm_result.action
                );

                return Ok(self.merge_results(
                    keyword_result,
                    llm_result,
                    "llm_inference".to_string(),
                ));
            } else {
                warn!("LLM inference failed, falling back to keyword filter");
            }
        }

        // 第三步: 默认使用关键词过滤结果
        Ok(ProcessingResult {
            should_show: keyword_result.should_show,
            method: "keyword_filter_fallback".to_string(),
            confidence: keyword_result.confidence,
            reason: keyword_result.reason,
            llm_analysis: None,
        })
    }

    /// 合并关键词过滤和 LLM 结果
    fn merge_results(
        &self,
        keyword: FilterResult,
        llm: LLMAnalysis,
        method: String,
    ) -> ProcessingResult {
        // 策略: LLM 置信度高则使用 LLM 结果，否则结合两者
        let should_show = if llm.confidence > self.config.llm_confidence_threshold {
            llm.action != "hide"
        } else {
            // 结合两个结果：都同意时最可信
            keyword.should_show && (llm.action != "hide")
        };

        let combined_confidence =
            (keyword.confidence + llm.confidence) / 2.0;

        ProcessingResult {
            should_show,
            method,
            confidence: combined_confidence,
            reason: format!(
                "Keyword: {} | LLM: {} (priority: {})",
                keyword.reason, llm.reasoning, llm.priority
            ),
            llm_analysis: Some(llm),
        }
    }
}

/// 通知处理结果
#[derive(Clone, Debug)]
pub struct ProcessingResult {
    /// 是否应该显示通知
    pub should_show: bool,
    /// 使用的处理方法
    pub method: String,
    /// 总体置信度
    pub confidence: f32,
    /// 决策理由
    pub reason: String,
    /// LLM 分析结果 (如果使用了 LLM)
    pub llm_analysis: Option<LLMAnalysis>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NotificationHandlerConfig::default();
        assert!(!config.enable_llm);
        assert!(config.enable_cache);
    }

    #[test]
    fn test_handler_creation() {
        let handler = HybridNotificationHandler::with_default_config();
        assert!(handler.llm_client.is_none());
    }

    #[tokio::test]
    async fn test_keyword_filter_priority() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        let result = handler
            .process_notification(
                "Critical Alert",
                "System is down",
                "SystemApp",
                &context,
            )
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.should_show);
        assert_eq!(result.method, "keyword_filter");
    }

    #[tokio::test]
    async fn test_spam_filtering() {
        let handler = HybridNotificationHandler::with_default_config();
        let context = UserContext::new();

        let result = handler
            .process_notification(
                "Limited Offer",
                "Click here for free prize",
                "AdApp",
                &context,
            )
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.should_show);
    }
}
