## 快速开始指南

### 1️⃣ 编译项目

```bash
cd /Users/jun/code/ai_notify
cargo build --release
```

编译完成后，二进制文件位置：
```
/Users/jun/code/ai_notify/target/release/ai_notify
```

### 2️⃣ 配置macOS权限

#### 方式A：图形界面
1. 打开 `系统设置` → `隐私与安全` → `辅助功能`
2. 点击 `+` 按钮
3. 选择编译后的二进制文件：`/Users/jun/code/ai_notify/target/release/ai_notify`
4. 点击 `打开`

#### 方式B：命令行
```bash
# 添加权限（需要sudo）
sudo chmod +x /Users/jun/code/ai_notify/target/release/ai_notify

# 可选：创建symbolic link方便调用
ln -s /Users/jun/code/ai_notify/target/release/ai_notify /usr/local/bin/ai_notify
```

### 3️⃣ 运行应用

#### 开发/调试模式（显示详细日志）
```bash
cargo run
```

日志输出示例：
```
2026-01-29T10:30:45.123Z INFO ai_notify: AI Notification Filter starting...
2026-01-29T10:30:45.234Z INFO ai_notify: Current context app=VSCode window_title="main.rs - ai_notify"
2026-01-29T10:30:45.456Z INFO ai_notify: Notification processed notification="Meeting: Sprint Planning" 
  should_show=true reason="Work-related notification in work context" confidence=0.85
```

#### 生产/后台模式
```bash
# 直接运行
./target/release/ai_notify

# 后台运行
./target/release/ai_notify &

# 加入后台并忽略输出
nohup ./target/release/ai_notify > /dev/null 2>&1 &
```

### 4️⃣ 检查运行状态

```bash
# 查看进程
ps aux | grep ai_notify

# 查看日志（如果重定向到文件）
tail -f ai_notify.log
```

### 5️⃣ 停止应用

```bash
# 查找进程ID
pgrep ai_notify

# 优雅关闭
kill -TERM <PID>

# 强制关闭
kill -9 <PID>
```

---

## 自定义配置

### 修改过滤规则

编辑 `src/filter.rs` 中的 `NotificationFilter::new()` 方法：

```rust
pub fn new() -> Self {
    Self {
        // 工作关键词（在工作上下文中优先显示）
        work_keywords: vec![
            "meeting".to_string(),
            "deadline".to_string(),
            "review".to_string(),
            "deploy".to_string(),
            "bug".to_string(),
            "issue".to_string(),
            "pull request".to_string(),
            "urgent".to_string(),
            // 添加你的自定义关键词...
        ],
        
        // 紧急关键词（任何上下文中都会显示）
        urgent_keywords: vec![
            "alert".to_string(),
            "critical".to_string(),
            "emergency".to_string(),
            "security".to_string(),
            "error".to_string(),
            "system down".to_string(),
            // 添加更多...
        ],
        
        // 垃圾关键词（任何上下文中都会过滤）
        spam_keywords: vec![
            "ad".to_string(),
            "advertisement".to_string(),
            "click here".to_string(),
            "spam".to_string(),
            "promotion".to_string(),
            // 添加更多...
        ],
    }
}
```

修改后重新编译：
```bash
cargo build --release
```

### 修改活动分类规则

编辑 `src/context.rs` 中的 `analyze_activity()` 方法：

```rust
pub fn analyze_activity(&self) -> ActivityType {
    let app_lower = self.app_name.to_lowercase();
    let title_lower = self.window_title.to_lowercase();

    // 工作关键词
    if Self::contains_any_keyword(&title_lower, &[
        "vscode", "intellij", "xcode", "github", "slack",
        // 添加你常用的工作应用...
    ]) {
        return ActivityType::Work;
    }

    // ... 类似地修改其他类别
    
    ActivityType::Unknown
}
```

### 调整轮询频率

编辑 `src/main.rs` 中的 `run()` 方法：

```rust
// 改变轮询间隔（毫秒）
sleep(Duration::from_millis(500)).await;  // 当前设置：500ms
sleep(Duration::from_millis(1000)).await; // 改为1秒（更节能）
sleep(Duration::from_millis(200)).await;  // 改为200ms（更快速）
```

---

## 常见问题

### Q1: 程序无法检测窗口

**问题**：显示 "Could not get current user context" 错误

**解决**：
1. 确保已在系统设置中授予 Accessibility 权限
2. 重启应用或重启macOS
3. 检查是否有其他应用在争夺焦点

### Q2: 通知没有被过滤

**问题**：所有通知都显示，过滤无效

**解决**：
1. 检查通知内容是否包含关键词（不区分大小写）
2. 查看日志输出确认活动类型分类
3. 修改关键词库使其更匹配你的用例

### Q3: CPU/内存占用高

**问题**：程序占用过多资源

**解决**：
1. 增加轮询间隔到 `1000ms` 或更高
2. 检查是否有其他程序干扰
3. 确保编译时使用了 `--release` 优化

### Q4: 权限错误

**问题**：显示 "Permission denied"

**解决**：
```bash
# 重新授权
chmod +x /Users/jun/code/ai_notify/target/release/ai_notify

# 或使用 sudo 重新添加到系统设置
sudo spctl --add /Users/jun/code/ai_notify/target/release/ai_notify
```

---

## 性能优化建议

### 1. 编译时优化
```bash
# 使用 LTO（链接时优化）
RUSTFLAGS="-C link-arg=-fuse-ld=lld -C target-cpu=native" cargo build --release
```

### 2. 运行时优化
- 增加轮询间隔（降低CPU占用）
- 缩小关键词库（降低匹配时间）
- 使用静态分配而非动态分配

### 3. 系统层面优化
```bash
# 设置进程优先级（较低优先级）
nice -n 10 ./target/release/ai_notify

# 或者
renice -n 10 <PID>
```

---

## 高级用法

### 创建 Launch Agent（开机自动启动）

1. 创建配置文件：
```bash
mkdir -p ~/.config/ai_notify
cat > ~/.config/ai_notify/ai_notify.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.local.ai-notify</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/jun/code/ai_notify/target/release/ai_notify</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/var/log/ai_notify.log</string>
    <key>StandardErrorPath</key>
    <string>/var/log/ai_notify_error.log</string>
</dict>
</plist>
EOF
```

2. 安装 Launch Agent：
```bash
cp ~/.config/ai_notify/ai_notify.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.local.ai-notify.plist
```

3. 验证状态：
```bash
launchctl list | grep ai-notify
```

### 集成到 cron 或 systemd

虽然macOS使用 LaunchAgent，但也可以通过 cron 定期检查：
```bash
# 编辑 crontab
crontab -e

# 添加行（每天早上9点启动）
0 9 * * * /Users/jun/code/ai_notify/target/release/ai_notify > /dev/null 2>&1 &
```

---

## 日志记录

### 启用详细日志

```bash
RUST_LOG=debug cargo run
```

日志级别：
- `TRACE`: 最详细（包括所有函数调用）
- `DEBUG`: 调试信息
- `INFO`: 信息消息（默认）
- `WARN`: 警告
- `ERROR`: 错误

### 将日志保存到文件

```bash
./target/release/ai_notify 2>&1 | tee -a ~/Library/Logs/ai_notify.log
```

---

## 下一步

- 阅读 [README_CN.md](README_CN.md) 了解更多架构细节
- 参考 [参考.md](参考.md) 查看技术背景
- 运行 `cargo test --test context_filtering -- --nocapture` 查看测试示例
- 开始自定义关键词和规则来适配你的工作流

祝使用愉快！ 🚀