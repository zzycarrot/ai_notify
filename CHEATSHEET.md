# 快速参考卡 (Cheat Sheet)

## 常用命令

### 开发
```bash
# 检查代码
cargo check

# 编译（开发）
cargo build

# 编译（发布）
cargo build --release

# 运行
cargo run

# 测试
cargo test

# 运行特定测试
cargo test test_work_context -- --nocapture

# 查看文档
cargo doc --open
```

### 代码质量
```bash
# 格式检查
cargo fmt --check

# 自动格式化
cargo fmt

# 代码检查（lints）
cargo clippy

# 检查安全漏洞
cargo audit
```

### 性能
```bash
# 性能分析
cargo build --release
time ./target/release/ai_notify

# 查看依赖大小
cargo tree

# 代码覆盖率
cargo tarpaulin
```

---

## 文件导航

| 文件 | 用途 |
|------|------|
| `src/lib.rs` | 库的公共接口 |
| `src/main.rs` | 主应用程序 |
| `src/context.rs` | 上下文检测与分类 |
| `src/filter.rs` | 通知过滤逻辑 |
| `src/notification.rs` | 系统通知集成 |
| `src/analyzer.rs` | 内容分析 |
| `tests/context_filtering.rs` | 集成测试 |
| `Cargo.toml` | 项目配置 |
| `README_CN.md` | 中文文档 |
| `USAGE.md` | 使用指南 |

---

## 关键概念

### ActivityType (活动类型)
```rust
pub enum ActivityType {
    Work,           // 工作应用
    Learning,       // 学习相关
    Entertainment,  // 娱乐应用
    Unknown,        // 未分类
}
```

### FilterResult (过滤结果)
```rust
pub struct FilterResult {
    pub should_show: bool,      // 是否显示
    pub reason: String,         // 原因
    pub confidence: f32,        // 置信度 (0.0-1.0)
}
```

### UserContext (用户上下文)
```rust
pub struct UserContext {
    pub app_name: String,       // 应用名称
    pub window_title: String,   // 窗口标题
}
```

---

## 常见修改

### 添加关键词

**工作关键词** (`src/filter.rs`):
```rust
work_keywords: vec![
    "meeting".to_string(),
    "YOUR_KEYWORD".to_string(),  // ← 添加这里
],
```

**紧急关键词**:
```rust
urgent_keywords: vec![
    "critical".to_string(),
    "YOUR_URGENT_KEYWORD".to_string(),  // ← 添加这里
],
```

**垃圾关键词**:
```rust
spam_keywords: vec![
    "ad".to_string(),
    "YOUR_SPAM_KEYWORD".to_string(),  // ← 添加这里
],
```

### 添加应用识别

在 `src/context.rs` 的 `analyze_activity()` 中:
```rust
if Self::contains_any_keyword(&app_lower, &[
    "vscode",
    "your_app",  // ← 添加应用
]) {
    return ActivityType::Work;
}
```

### 调整过滤策略

在 `src/filter.rs` 中，修改对应的过滤方法:
```rust
fn filter_work_context(&self, notification_text: &str) -> FilterResult {
    // 修改这里的逻辑
}
```

---

## 日志输出示例

### 正常运行
```
2026-01-29T10:30:45.123Z INFO ai_notify: AI Notification Filter starting...
2026-01-29T10:30:45.234Z INFO ai_notify: Current context app=VSCode window_title="main.rs"
2026-01-29T10:30:45.456Z INFO ai_notify: Notification processed 
  notification="Meeting at 2PM" should_show=true confidence=0.85
```

### 调试模式
```
RUST_LOG=debug cargo run
```

---

## 故障排查

### 编译错误
```bash
# 清理缓存
cargo clean

# 重新构建
cargo build
```

### 权限问题
```bash
# 检查权限
ls -l target/release/ai_notify

# 修复权限
chmod +x target/release/ai_notify

# 在系统设置中重新添加
# 系统设置 → 隐私与安全 → 辅助功能 → 添加
```

### 高内存占用
```bash
# 增加轮询间隔
# 编辑 src/main.rs 中的 sleep(Duration::...)

# 减少关键词库
# 编辑 src/filter.rs 中的 new() 方法
```

### 没有检测到通知
```bash
# 1. 检查权限
launchctl list | grep accessibility

# 2. 重启应用
killall ai_notify
./target/release/ai_notify

# 3. 查看日志
RUST_LOG=debug cargo run
```

---

## 性能目标

| 指标 | 目标 | 当前 |
|-----|------|------|
| 启动时间 | <200ms | <100ms ✅ |
| 检测延迟 | <1s | <500ms ✅ |
| 内存占用 | <50MB | ~15-20MB ✅ |
| CPU占用 | <2% | <1% ✅ |
| 二进制大小 | <5MB | 1.1MB ✅ |

---

## 测试命令

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_work_context

# 显示输出
cargo test -- --nocapture

# 并行测试
cargo test -- --test-threads=4

# 单线程测试
cargo test -- --test-threads=1
```

### 测试覆盖率
```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

---

## 发布检查清单

- [ ] 所有测试通过 (`cargo test`)
- [ ] 无警告 (`cargo clippy`)
- [ ] 代码格式化 (`cargo fmt`)
- [ ] 安全检查 (`cargo audit`)
- [ ] 性能基准 (`cargo build --release`)
- [ ] 文档更新 (README, USAGE)
- [ ] 版本号更新 (Cargo.toml)

---

## 部署

### 编译发布版本
```bash
cargo build --release
# 输出: target/release/ai_notify
```

### 复制到系统路径
```bash
sudo cp target/release/ai_notify /usr/local/bin/
chmod +x /usr/local/bin/ai_notify
```

### 创建 Launch Agent（开机启动）
```bash
mkdir -p ~/Library/LaunchAgents
# 编辑 com.local.ai-notify.plist (参考 USAGE.md)
launchctl load ~/Library/LaunchAgents/com.local.ai-notify.plist
```

---

## 有用的链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Tokio 文档](https://tokio.rs/)
- [macOS 开发指南](https://developer.apple.com/macos/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

## 快速提示

💡 **Tip 1**: 修改代码后，运行 `cargo fmt` 自动格式化

💡 **Tip 2**: 使用 `RUST_LOG=debug cargo run` 查看详细日志

💡 **Tip 3**: `cargo clippy` 能发现大多数常见错误

💡 **Tip 4**: `cargo test -- --nocapture` 显示 println! 输出

💡 **Tip 5**: 编辑 `Cargo.toml` 中的 `[profile.release]` 优化编译

---

**更新时间**: 2026-01-29