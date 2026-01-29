/// LLM (Large Language Model) 集成模块
/// 集成 Qwen3-0.6B 模型用于高级通知分析

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// LLM 推理配置
#[derive(Clone, Debug)]
pub struct LLMConfig {
    /// Hugging Face 模型 ID
    pub model_id: String,
    /// 是否启用思考模式 (thinking mode)
    pub enable_thinking: bool,
    /// 最大生成令牌数
    pub max_tokens: usize,
    /// 温度参数 (0.0 - 2.0)
    pub temperature: f32,
    /// Top-P 采样参数
    pub top_p: f32,
    /// 是否启用本地模型
    pub local_mode: bool,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_id: "Qwen/Qwen3-0.6B".to_string(),
            enable_thinking: false, // 非思考模式以提高速度
            max_tokens: 512,        // 轻量级推理
            temperature: 0.7,
            top_p: 0.8,
            local_mode: false,
        }
    }
}

/// LLM 分析结果
#[derive(Clone, Debug)]
pub struct LLMAnalysis {
    /// 通知优先级 (1-10，10最高)
    pub priority: u8,
    /// 通知分类 (work/personal/spam/urgent/other)
    pub category: String,
    /// 推荐行动 (show/hide/defer)
    pub action: String,
    /// 置信度 (0.0-1.0)
    pub confidence: f32,
    /// LLM 生成的理由
    pub reasoning: String,
}

/// LLM 客户端 (基于 Qwen3-0.6B)
pub struct LLMClient {
    config: LLMConfig,
    // 在实际实现中，这里会存储模型加载器或 API 客户端
    // 目前是占位符，等待 MLX Rust 绑定或 onnxruntime-rs
}

impl LLMClient {
    pub fn new(config: LLMConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(LLMConfig::default())
    }

    /// 分析通知并返回 LLM 推理结果
    /// 
    /// # 参数
    /// - `notification_title`: 通知标题
    /// - `notification_body`: 通知正文
    /// - `app_name`: 应用名称
    /// - `current_activity`: 当前用户活动上下文
    ///
    /// # 返回
    /// LLMAnalysis 结构体包含分析结果
    pub async fn analyze_notification(
        &self,
        notification_title: &str,
        notification_body: &str,
        app_name: &str,
        current_activity: &str,
    ) -> Result<LLMAnalysis> {
        // 构造提示词
        let prompt = self.build_analysis_prompt(
            notification_title,
            notification_body,
            app_name,
            current_activity,
        );

        // 调用 LLM 推理
        let response = self.call_llm(&prompt).await?;

        // 解析 LLM 响应
        let analysis = self.parse_llm_response(&response)?;

        Ok(analysis)
    }

    /// 构造分析提示词
    fn build_analysis_prompt(
        &self,
        title: &str,
        body: &str,
        app: &str,
        activity: &str,
    ) -> String {
        format!(
            r#"You are a smart notification filtering assistant for macOS. Analyze the following notification and respond with a JSON object.

Notification Details:
- Title: {}
- Body: {}
- App: {}
- Current User Activity: {}

Analyze this notification and respond ONLY with a JSON object (no markdown, no extra text) containing:
{{
    "priority": <1-10, where 10 is most important>,
    "category": "<work|personal|spam|urgent|other>",
    "action": "<show|hide|defer>",
    "confidence": <0.0-1.0>,
    "reasoning": "<brief explanation>"
}}

Consider:
1. If user is working (coding, meetings), prioritize work-related notifications
2. If user is learning, allow educational content and work alerts
3. If user is entertaining, show urgent alerts only
4. Spam/ads should be hidden regardless of context
5. Critical/security alerts should always be shown

Respond with ONLY the JSON object."#,
            title, body, app, activity
        )
    }

    /// 调用 LLM 推理接口
    async fn call_llm(&self, prompt: &str) -> Result<String> {
        if self.config.local_mode {
            self.call_local_llm(prompt).await
        } else {
            self.call_huggingface_api(prompt).await
        }
    }

    /// 本地 LLM 推理 (需要 MLX 或 ONNX Runtime)
    async fn call_local_llm(&self, _prompt: &str) -> Result<String> {
        // 占位符：等待 MLX Rust 绑定
        // 这里将集成 mlx-community/mlx-rs 或 onnxruntime-rs
        // 
        // 预期实现:
        // 1. 加载模型: mlx::Model::load(&self.config.model_id)?
        // 2. 预处理输入: tokenizer.encode(prompt)?
        // 3. 运行推理: model.forward(&tokens)?
        // 4. 后处理输出: tokenizer.decode(&output)?
        
        Err(anyhow::anyhow!(
            "Local LLM not yet implemented. Awaiting MLX Rust bindings."
        ))
    }

    /// 调用 Hugging Face Inference API
    async fn call_huggingface_api(&self, _prompt: &str) -> Result<String> {
        // 占位符：集成 HF Inference API
        // 预期实现:
        // 1. 从环境变量读取 HF_TOKEN
        // 2. 使用 reqwest 发送 POST 请求到 HF API
        // 3. 解析响应并返回生成的文本
        
        Err(anyhow::anyhow!(
            "Hugging Face API integration not yet implemented. \
            Set HF_TOKEN environment variable when ready."
        ))
    }

    /// 解析 LLM 响应
    fn parse_llm_response(&self, response: &str) -> Result<LLMAnalysis> {
        // 尝试从响应中提取 JSON
        let json_str = self.extract_json(response)?;
        let json: serde_json::Value = serde_json::from_str(&json_str)?;

        let analysis = LLMAnalysis {
            priority: json["priority"]
                .as_u64()
                .unwrap_or(5)
                .min(10) as u8,
            category: json["category"]
                .as_str()
                .unwrap_or("other")
                .to_string(),
            action: json["action"]
                .as_str()
                .unwrap_or("show")
                .to_string(),
            confidence: json["confidence"]
                .as_f64()
                .unwrap_or(0.5) as f32,
            reasoning: json["reasoning"]
                .as_str()
                .unwrap_or("No reasoning provided")
                .to_string(),
        };

        Ok(analysis)
    }

    /// 从响应中提取 JSON 对象
    fn extract_json(&self, response: &str) -> Result<String> {
        // 查找第一个 '{' 和最后一个 '}'
        let start = response
            .find('{')
            .ok_or_else(|| anyhow::anyhow!("No JSON object found in response"))?;
        let end = response
            .rfind('}')
            .ok_or_else(|| anyhow::anyhow!("No JSON object found in response"))?;

        Ok(response[start..=end].to_string())
    }
}

/// 轻量级 LLM 缓存，用于减少 API 调用
pub struct LLMCache {
    cache: Arc<RwLock<std::collections::HashMap<String, LLMAnalysis>>>,
}

impl LLMCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 生成缓存键
    pub fn make_key(title: &str, body: &str, app: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        (title, body, app).hash(&mut hasher);
        format!("llm_{:x}", hasher.finish())
    }

    /// 从缓存获取分析结果
    pub async fn get(&self, key: &str) -> Option<LLMAnalysis> {
        self.cache.read().await.get(key).cloned()
    }

    /// 将分析结果存入缓存
    pub async fn set(&self, key: String, analysis: LLMAnalysis) {
        self.cache.write().await.insert(key, analysis);
    }

    /// 清空缓存
    pub async fn clear(&self) {
        self.cache.write().await.clear();
    }

    /// 获取缓存大小
    pub async fn size(&self) -> usize {
        self.cache.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LLMConfig::default();
        assert_eq!(config.model_id, "Qwen/Qwen3-0.6B");
        assert!(!config.enable_thinking);
        assert_eq!(config.temperature, 0.7);
    }

    #[test]
    fn test_json_extraction() {
        let client = LLMClient::new(LLMConfig::default());
        let response = r#"Some text before {"priority": 8, "category": "work"} some text after"#;
        let json = client.extract_json(response);
        assert!(json.is_ok());
        assert!(json.unwrap().contains("priority"));
    }

    #[test]
    fn test_prompt_building() {
        let client = LLMClient::new(LLMConfig::default());
        let prompt = client.build_analysis_prompt(
            "Test Title",
            "Test Body",
            "TestApp",
            "working",
        );
        assert!(prompt.contains("Test Title"));
        assert!(prompt.contains("working"));
    }

    #[test]
    fn test_cache_key_generation() {
        let key1 = LLMCache::make_key("title", "body", "app");
        let key2 = LLMCache::make_key("title", "body", "app");
        let key3 = LLMCache::make_key("title", "body", "other");

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = LLMCache::new();
        let key = "test_key".to_string();
        let analysis = LLMAnalysis {
            priority: 8,
            category: "work".to_string(),
            action: "show".to_string(),
            confidence: 0.95,
            reasoning: "Test".to_string(),
        };

        cache.set(key.clone(), analysis.clone()).await;
        let retrieved = cache.get(&key).await;

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().priority, 8);
    }
}
