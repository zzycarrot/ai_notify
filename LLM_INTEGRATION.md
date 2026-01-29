# Qwen3-0.6B LLM 集成指南

## 概述

本项目已集成 **Qwen3-0.6B** 模型以增强通知筛选的智能性。这是一个轻量级的 0.6B 参数模型，专为边缘设备设计，支持推理和非推理模式。

### 模型特性

- **参数量**: 0.6B (600M parameters)
- **非嵌入参数**: 440M 
- **层数**: 28
- **上下文长度**: 32,768 tokens
- **两种模式**:
  - 🧠 **思考模式** (`enable_thinking=true`): 用于复杂推理，生成 `<think>...</think>` 块
  - ⚡ **高效模式** (`enable_thinking=false`): 用于快速响应，类似 Qwen2.5-Instruct

### 架构设计

```
混合通知处理器 (Hybrid Handler)
    ├── 快速路径 (关键词过滤) <10ms, 本地
    ├── 标准路径 (LLM 推理) <500ms, 可配置
    └── 缓存层 (避免重复推理)
```

## 使用方法

### 1. 启用 LLM 推理（基础配置）

```rust
use ai_notify::engine::{HybridNotificationHandler, NotificationHandlerConfig};
use ai_notify::llm::LLMConfig;

// 创建 LLM 配置
let llm_config = LLMConfig {
    model_id: "Qwen/Qwen3-0.6B".to_string(),
    enable_thinking: false,  // 禁用思考模式以提高速度
    max_tokens: 512,         // 轻量级推理
    temperature: 0.7,
    top_p: 0.8,
    local_mode: false,       // 使用 Hugging Face API
};

// 创建处理器配置
let config = NotificationHandlerConfig {
    enable_llm: true,                      // 启用 LLM
    llm_confidence_threshold: 0.7,         // 置信度阈值
    enable_cache: true,                    // 启用缓存
    llm_config,
};

// 创建混合处理器
let handler = HybridNotificationHandler::new(config);

// 处理通知
let result = handler.process_notification(
    "Code Review",
    "Your PR has been reviewed",
    "GitHub",
    &context,
).await?;

println!("Should show: {}", result.should_show);
println!("Method: {}", result.method);
println!("Confidence: {}", result.confidence);
```

### 2. 本地模型推理（高级配置）

当 MLX Rust 或 ONNX Runtime 支持可用时：

```rust
let llm_config = LLMConfig {
    model_id: "Qwen/Qwen3-0.6B".to_string(),
    enable_thinking: false,
    max_tokens: 512,
    temperature: 0.7,
    top_p: 0.8,
    local_mode: true,  // ← 启用本地推理
};
```

### 3. 启用思考模式（复杂推理）

对于需要逻辑推理的复杂通知：

```rust
let llm_config = LLMConfig {
    model_id: "Qwen/Qwen3-0.6B".to_string(),
    enable_thinking: true,  // ← 启用思考模式
    max_tokens: 1024,       // 增加输出空间
    temperature: 0.6,       // 推荐温度
    top_p: 0.95,           // 推荐 top_p
    local_mode: false,
};
```

## API 集成方式

### Hugging Face Inference API

需要设置环境变量：

```bash
export HF_TOKEN=hf_xxxxxxxxxxxxxxxxxxxx
```

获取 Token: https://huggingface.co/settings/tokens

### SGLang (推荐用于 API)

```bash
python -m sglang.launch_server \
    --model-path Qwen/Qwen3-0.6B \
    --reasoning-parser qwen3
```

### vLLM

```bash
vllm serve Qwen/Qwen3-0.6B \
    --enable-reasoning \
    --reasoning-parser deepseek_r1
```

### 本地部署选项

- **Ollama**: 支持 Qwen3-0.6B
- **LMStudio**: 图形界面支持
- **MLX-LM**: macOS 优化（待 Rust 绑定）
- **llama.cpp**: C++ 推理引擎
- **KTransformers**: 高性能推理

## LLM 分析结果

`LLMAnalysis` 结构体包含：

```rust
pub struct LLMAnalysis {
    /// 优先级 (1-10，10最高)
    pub priority: u8,
    
    /// 分类 (work|personal|spam|urgent|other)
    pub category: String,
    
    /// 推荐行动 (show|hide|defer)
    pub action: String,
    
    /// 置信度 (0.0-1.0)
    pub confidence: f32,
    
    /// LLM 的推理理由
    pub reasoning: String,
}
```

## 性能最佳实践

### 采样参数

**高效模式** (`enable_thinking=false`):
- Temperature: 0.7
- Top-P: 0.8
- Top-K: 20
- Min-P: 0

**思考模式** (`enable_thinking=true`):
- Temperature: 0.6
- Top-P: 0.95
- Top-K: 20
- Min-P: 0

### 缓存策略

启用缓存以避免重复推理：

```rust
let cache = LLMCache::new();

// 生成缓存键
let key = LLMCache::make_key("title", "body", "app_name");

// 获取缓存结果
if let Some(cached) = cache.get(&key).await {
    // 使用缓存结果
}

// 设置缓存
cache.set(key, analysis).await;
```

## 混合处理策略

处理器使用以下优先级：

1. **关键词过滤** (本地, <10ms)
   - 高置信度情况 (>0.85) 直接返回
   - 包括紧急告警和垃圾检测

2. **LLM 推理** (API/本地, <500ms)
   - 关键词置信度不足时调用
   - 使用缓存避免重复调用
   - 与关键词结果结合

3. **融合结果**
   - 关键词 + LLM 双重确认
   - 加权置信度计算

## 测试用例

```rust
#[tokio::test]
async fn test_hybrid_notification_handling() {
    let config = NotificationHandlerConfig {
        enable_llm: true,
        llm_confidence_threshold: 0.7,
        enable_cache: true,
        llm_config: LLMConfig::default(),
    };
    
    let handler = HybridNotificationHandler::new(config);
    let context = UserContext::new();
    
    let result = handler.process_notification(
        "Code Review Ready",
        "Your pull request has 2 approvals",
        "GitHub",
        &context,
    ).await.unwrap();
    
    assert!(result.should_show);
    assert_eq!(result.method, "llm_inference");
}
```

## 路线图

### Phase 1 (当前): ✅ 基础框架
- [x] LLM 配置系统
- [x] 提示词模板
- [x] 结果解析
- [x] 缓存机制
- [x] 混合处理引擎

### Phase 2: 🔄 API 集成
- [ ] Hugging Face Inference API
- [ ] OpenAI 兼容 API 支持
- [ ] 请求队列和限流
- [ ] 错误重试机制

### Phase 3: 📱 本地推理
- [ ] MLX Rust 绑定 (macOS GPU 加速)
- [ ] ONNX Runtime 支持 (跨平台)
- [ ] 模型量化 (减少内存占用)
- [ ] 批处理推理

### Phase 4: 🧠 高级功能
- [ ] 用户学习系统 (自适应权重)
- [ ] 多模型集成 (备选方案)
- [ ] 性能监控和分析
- [ ] 远程配置管理

## 故障排除

### LLM 推理失败回退

如果 LLM 不可用，系统自动回退到关键词过滤：

```rust
// 自动处理
if llm_inference_fails {
    // 使用关键词过滤作为后备方案
    return keyword_filter_result;
}
```

### 常见问题

**Q: 如何禁用 LLM？**
```rust
let config = NotificationHandlerConfig {
    enable_llm: false,  // ← 禁用
    ..Default::default()
};
```

**Q: 缓存大小多大？**
```rust
let size = cache.size().await;  // 返回缓存条目数
cache.clear().await;             // 清空缓存
```

**Q: 支持离线模式吗？**

支持。使用本地模型推理（需要 MLX/ONNX Runtime）。

## 参考资源

- [Qwen3 官方文档](https://qwen.readthedocs.io/)
- [Hugging Face 模型卡](https://huggingface.co/Qwen/Qwen3-0.6B)
- [Qwen3 技术报告](https://arxiv.org/abs/2505.09388)
- [GitHub 仓库](https://github.com/QwenLM/Qwen3)

## 许可证

Qwen3-0.6B 模型采用 Apache License 2.0

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**最后更新**: 2026年1月30日
