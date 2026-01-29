# AI Notification Filter - 基于上下文的macOS通知筛选工具

一个用Rust开发的智能通知筛选系统，可以根据用户当前的活动上下文（工作、学习、娱乐）来自动决定是否显示系统通知。

## 功能特性

### 🎯 核心功能
- **上下文感知**：自动检测用户当前使用的应用和窗口标题
- **活动分类**：将用户活动分为工作、学习、娱乐和未知四种类型
- **智能过滤**：根据上下文类型对通知进行适应性筛选
- **关键词识别**：内置了工作、学习、娱乐等多个领域的关键词库

### 📱 上下文分析示例

#### 工作上下文 (Work)
- VSCode、IntelliJ、Xcode、Visual Studio
- GitHub、GitLab、Jira
- Slack、Microsoft Teams
- Confluence、Notion
- Office 相关应用

在工作上下文中：
- ✅ **显示**：工作相关通知（会议、deadline、代码review等）
- ✅ **显示**：个人消息（但优先级较低）
- ✅ **显示**：紧急警报

#### 学习上下文 (Learning)
- Chrome/Firefox/Safari 打开文档、教程、书籍
- Stack Overflow、GitHub文档
- Coursera、Udemy、Khan Academy等教育平台

在学习上下文中：
- ✅ **显示**：学习相关通知
- ✅ **显示**：工作相关通知（优先级高）
- ❌ **过滤**：娱乐分心通知（游戏、视频等）

#### 娱乐上下文 (Entertainment)
- Bilibili、YouTube、Netflix
- 游戏相关（LOL、Dota、Steam等）
- 社交媒体（Twitter、Discord、QQ等）

在娱乐上下文中：
- ✅ **显示**：所有非垃圾通知
- ❌ **过滤**：仅过滤垃圾/广告通知

### ⚡ 关键特性
- 轻量级架构（Rust编译到单个二进制文件）
- 无需Python运行时
- 低CPU占用，高效的内存管理
- 实时上下文检测（500ms轮询间隔）
- 可扩展的关键词配置

## 架构设计

```
┌─────────────────────────────────────────┐
│     macOS Notification Center            │
└────────────┬────────────────────────────┘
             │ (System Notifications)
             ▼
┌─────────────────────────────────────────┐
│  NotificationListener (Thread)           │
│  - Monitors system notifications         │
└────────────┬────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────┐
│  NotificationEngine (Async Runtime)      │
│  ┌─────────────────────────────────────┐ │
│  │ 1. Get Current Window Context        │ │
│  │    (active-win-pos-rs)              │ │
│  └─────────────────────────────────────┘ │
│  ┌─────────────────────────────────────┐ │
│  │ 2. Classify Activity Type            │ │
│  │    (Work/Learning/Entertainment)    │ │
│  └─────────────────────────────────────┘ │
│  ┌─────────────────────────────────────┐ │
│  │ 3. Apply NotificationFilter          │ │
│  │    (Rule-based & Keyword-based)     │ │
│  └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
             │
    ┌────────┴────────┐
    ▼                 ▼
  SHOW          FILTER OUT
(Display)       (Suppress)
```

## 项目结构

```
src/
├── lib.rs              # 库入口，导出所有模块
├── main.rs             # 主应用程序
├── context.rs          # 用户上下文检测和活动分类
├── filter.rs           # 通知过滤逻辑
├── notification.rs     # macOS通知系统集成
└── analyzer.rs         # 通知内容分析
```

### 核心模块说明

#### context.rs - 上下文管理
- `UserContext`：代表用户当前活动
- `ActivityType`：活动分类枚举
- `analyze_activity()`：根据应用和窗口标题分类

#### filter.rs - 过滤引擎
- `NotificationFilter`：核心过滤器
- `FilterResult`：过滤结果（显示/隐藏 + 原因 + 置信度）
- 支持上下文感知的过滤规则

#### notification.rs - 系统集成
- `SystemNotification`：系统通知数据结构
- `NotificationListener`：监听系统通知的线程

## 安装与使用

### 前置条件
- macOS 12+（支持native Arm64）
- Rust 1.70+
- Accessibility权限（用于窗口检测）

### 编译
```bash
cd /Users/jun/code/ai_notify
cargo build --release
```

### 授予Accessibility权限
1. 打开 **系统设置** → **隐私与安全** → **辅助功能**
2. 添加编译后的二进制文件：
   ```
   /Users/jun/code/ai_notify/target/release/ai_notify
   ```

### 运行
```bash
# 开发模式（带调试日志）
cargo run

# 发布模式（优化版本）
./target/release/ai_notify

# 后台运行
./target/release/ai_notify &
```

### 查看日志
默认的日志级别是 `INFO`。修改 `src/main.rs` 中的 `with_max_level()` 可以改变日志详度：
```rust
.with_max_level(tracing::Level::DEBUG)  // 详细调试信息
.with_max_level(tracing::Level::WARN)   // 仅警告和错误
```

## 配置

### 修改关键词库
编辑 `src/filter.rs` 中的 `NotificationFilter::new()` 方法：

```rust
pub fn new() -> Self {
    Self {
        work_keywords: vec![
            "meeting".to_string(),
            "deadline".to_string(),
            // 添加更多关键词...
        ],
        // ...
    }
}
```

### 调整轮询间隔
在 `src/main.rs` 中修改 `sleep()` 的时间：
```rust
sleep(Duration::from_millis(500)).await;  // 当前设置
sleep(Duration::from_secs(1)).await;      // 改为1秒
```

## 未来扩展方向

### 1. MLX 模型集成 (已规划)
集成Qwen3-0.6B或类似的轻量级模型进行更智能的文本分析：
```rust
// 伪代码示例
let model = MLXModel::load("qwen3-0.6b")?;
let relevance_score = model.infer(&notification_text)?;
```

### 2. 本地学习 (Planned)
根据用户的过滤历史自动调整关键词和规则。

### 3. GUI配置工具 (Planned)
提供友好的图形界面来配置过滤规则。

### 4. Slack/Teams 集成 (Planned)
直接与工作应用集成，提供更精确的优先级判断。

## 性能指标

- **内存占用**：~15-20 MB（包括所有依赖）
- **CPU占用**：<1% 空闲状态
- **启动时间**：<100ms
- **通知检测延迟**：<500ms

## 调试

### 启用详细日志
运行时设置环境变量：
```bash
RUST_LOG=debug cargo run
```

### 测试特定上下文
手动修改 `src/main.rs` 中的演示通知：
```rust
let demo_notification = SystemNotification {
    title: "Test: Code Review".to_string(),
    body: "PR #123 needs your review".to_string(),
    app_name: "Slack".to_string(),
    timestamp: std::time::SystemTime::now(),
};
```

## 技术栈

| 组件 | 库 | 用途 |
|------|-----|------|
| 窗口检测 | `active-win-pos-rs` | 获取当前活动窗口 |
| Async运行时 | `tokio` | 异步任务管理 |
| 日志系统 | `tracing`, `tracing-subscriber` | 结构化日志 |
| macOS API | `objc2`, `objc2-foundation` | 与系统集成 |
| JSON | `serde_json` | 数据序列化 |

## MLX 模型集成指南 (未来)

### 集成步骤

1. **添加MLX依赖** (当官方Rust绑定可用时)
```toml
[dependencies]
mlx = "0.1"  # 待发布
```

2. **模型加载**
```rust
use ai_notify::MLXModel;

let model = MLXModel::load("qwen3-0.6b")?;
```

3. **文本分类**
```rust
let confidence = model.classify_notification(&notification_text, &context)?;
if confidence > 0.7 {
    // 显示通知
}
```

## 许可证

MIT License

## 参考资源

- [Candle框架](https://github.com/huggingface/candle)
- [MLX框架](https://github.com/ml-explore/mlx)
- [Active Window Detection](https://github.com/sioodmy/active-win-pos-rs)
- [Rust macOS开发](https://docs.rs/objc2/)
- [参考技术文档](./参考.md)