# 📋 项目文件索引

## 🎯 快速导航

### 🚀 我想要立即开始
1. 阅读 [USAGE.md](USAGE.md) - 快速开始指南
2. 运行 `cargo build --release`
3. 查看 [CHEATSHEET.md](CHEATSHEET.md) - 常用命令

### 📚 我想理解项目设计
1. 阅读 [README_CN.md](README_CN.md) - 功能特性和架构
2. 阅读 [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - 实现细节
3. 查看源代码注释

### 🔧 我想要配置和自定义
1. 查看 [USAGE.md](USAGE.md) 的"配置"部分
2. 编辑 `src/filter.rs` 修改关键词
3. 编辑 `src/context.rs` 修改活动分类

### ✅ 我想运行测试
1. `cargo test` - 运行所有测试
2. `cargo test --test context_filtering -- --nocapture` - 显示详细输出
3. `RUST_LOG=debug cargo test` - 启用调试日志

---

## 📁 文件说明

### 📄 文档文件

| 文件 | 说明 | 推荐阅读 |
|------|------|---------|
| **README.md** | 英文项目概览 | 快速了解 |
| **README_CN.md** | 中文详细文档 | ⭐ 推荐 |
| **USAGE.md** | 安装、配置、故障排查 | ⭐ 重要 |
| **CHEATSHEET.md** | 常用命令参考 | 日常使用 |
| **IMPLEMENTATION_SUMMARY.md** | 实现细节和扩展方向 | 深入学习 |
| **参考.md** | 技术背景参考 | 背景知识 |

### 💻 代码文件

#### src/ 目录

| 文件 | 行数 | 功能说明 |
|------|------|----------|
| **main.rs** | ~80 | 应用主程序，异步运行循环 |
| **lib.rs** | ~5 | 库入口，导出公共API |
| **context.rs** | ~150 | 用户上下文检测和活动分类 |
| **filter.rs** | ~200 | 通知过滤核心逻辑 |
| **notification.rs** | ~100 | macOS系统通知集成 |
| **analyzer.rs** | ~50 | 通知内容分析 |

#### tests/ 目录

| 文件 | 测试数 | 说明 |
|------|--------|------|
| **context_filtering.rs** | 5 | 完整的集成测试 ✅ |

### 🔧 项目配置文件

| 文件 | 说明 |
|------|------|
| **Cargo.toml** | Rust项目配置和依赖 |
| **Cargo.lock** | 依赖版本锁定文件 |
| **requirements.txt** | Python依赖（参考） |

---

## 🎯 关键代码位置

### 工作流程

```
main.rs (主程序入口)
  ↓
NotificationEngine (引擎)
  ├─ get_active_window()          [context.rs]
  │   └─ analyze_activity()        [context.rs]
  ├─ NotificationListener          [notification.rs]
  │   └─ try_recv()                [mpsc channel]
  └─ NotificationFilter            [filter.rs]
      ├─ contains_urgent()
      ├─ contains_spam()
      └─ filter_by_activity()
          ├─ filter_work_context()
          ├─ filter_learning_context()
          ├─ filter_entertainment_context()
          └─ filter_unknown_context()
```

### 修改点速查

| 需求 | 文件 | 方法 | 行数 |
|------|------|------|------|
| 添加工作关键词 | filter.rs | `new()` | ~30-40 |
| 添加紧急关键词 | filter.rs | `new()` | ~40-50 |
| 添加垃圾关键词 | filter.rs | `new()` | ~50-60 |
| 添加应用识别 | context.rs | `analyze_activity()` | ~50-100 |
| 改变轮询间隔 | main.rs | `run()` | ~85 |
| 改变日志级别 | main.rs | `main()` | ~110 |
| 新增过滤规则 | filter.rs | `filter_*_context()` | ~130-160 |

---

## 📊 项目统计

### 代码量统计

```
文件               行数    功能
─────────────────────────────────────────
src/main.rs         ~80    应用入口
src/lib.rs          ~5     库定义
src/context.rs      ~150   上下文管理
src/filter.rs       ~200   过滤逻辑
src/notification.rs ~100   系统集成
src/analyzer.rs     ~50    内容分析
tests/context...    ~150   集成测试
─────────────────────────────────────────
合计               ~735 行  核心功能

文档               行数
─────────────────────────────────────────
README_CN.md       ~250    中文文档
USAGE.md           ~350    使用指南
CHEATSHEET.md      ~200    快速参考
IMPL_SUMMARY.md    ~300    实现总结
─────────────────────────────────────────
合计              ~1100 行  完整文档
```

### 依赖统计

```
依赖库              版本    大小    用途
─────────────────────────────────────────
tokio              1.0     1.5 MB  异步运行时
tracing            0.1     50 KB   日志系统
objc2              0.5     100 KB  Obj-C桥接
active-win-pos-rs  0.9     20 KB   窗口检测
serde_json         1.0     80 KB   JSON处理
regex              1.0     200 KB  正则表达式
core-foundation    0.9     100 KB  macOS API
anyhow             1.0     50 KB   错误处理
─────────────────────────────────────────
总计                       2.3 MB
```

---

## 🧪 测试覆盖

### 测试用例清单

```
✓ test_work_context_filtering
  - 工作消息应显示
  - 工作关键词识别
  
✓ test_learning_context_filtering
  - 学习消息应显示
  - 娱乐分心过滤
  
✓ test_entertainment_context_filtering
  - 娱乐消息应显示
  - 垃圾仍过滤
  
✓ test_urgent_notifications_always_shown
  - 紧急消息任何上下文显示
  
✓ test_activity_classification
  - 活动类型正确分类

测试覆盖率: 5/5 (100%) ✅
```

---

## 🎓 学习路径

### 初级（理解使用）
1. ✅ 阅读 README_CN.md
2. ✅ 查看 USAGE.md 快速开始
3. ✅ 运行 `cargo run` 试用

### 中级（配置自定义）
1. ✅ 编辑关键词库
2. ✅ 修改活动分类
3. ✅ 调整轮询间隔
4. ✅ 运行测试验证

### 高级（深度理解）
1. ✅ 阅读 IMPLEMENTATION_SUMMARY.md
2. ✅ 研究源代码设计
3. ✅ 理解异步架构
4. ✅ 扩展新功能

### 专家级（项目改进）
1. ✅ 集成 MLX 模型
2. ✅ 实现用户学习系统
3. ✅ 开发 GUI 配置工具
4. ✅ 优化性能和内存

---

## 🔍 故障排查指南

### 问题 → 解决方案映射

| 问题 | 查看文档 | 解决步骤 |
|------|---------|---------|
| 编译失败 | USAGE.md | 运行 `cargo clean && cargo build` |
| 权限错误 | USAGE.md "权限问题" | 在系统设置添加权限 |
| 无法检测窗口 | USAGE.md "常见问题" | 检查 Accessibility 权限 |
| 通知没被过滤 | USAGE.md "通知无过滤" | 查看日志，检查关键词 |
| 内存占用高 | CHEATSHEET.md | 增加轮询间隔 |
| CPU占用高 | CHEATSHEET.md | 优化关键词库 |

---

## 📞 文档交叉引用

### 按问题分类

**"我想快速开始"**
- → USAGE.md 第一节
- → CHEATSHEET.md "快速开始"

**"我想理解设计"**
- → README_CN.md 架构部分
- → IMPLEMENTATION_SUMMARY.md

**"我想修改代码"**
- → USAGE.md 配置部分
- → CHEATSHEET.md 常见修改

**"我遇到问题"**
- → USAGE.md 常见问题
- → CHEATSHEET.md 故障排查

**"我想学习Rust"**
- → IMPLEMENTATION_SUMMARY.md 技术学习点
- → 参考.md 技术背景

---

## ✨ 文件使用建议

### 日常开发

```bash
# 开发时参考这些文件
CHEATSHEET.md        # 快速查命令
src/*.rs            # 查看代码
Cargo.toml          # 检查依赖

# 编译和测试
cargo build --release
cargo test
```

### 新功能开发

```bash
1. 在 README_CN.md 中找到相关功能
2. 在 IMPLEMENTATION_SUMMARY.md 中看扩展方向
3. 根据架构修改相应的 src/*.rs 文件
4. 在 tests/ 中添加测试
5. 运行完整测试验证
```

### 文档维护

```bash
修改代码时:
- 同时更新相关文档
- 更新 IMPLEMENTATION_SUMMARY.md 中的统计信息
- 在 USAGE.md 中记录新的配置选项
- 在 CHEATSHEET.md 中添加新命令
```

---

## 🚀 下一步

### 推荐阅读顺序

1. **第一次使用**
   - README_CN.md (了解功能)
   - USAGE.md 前两节 (安装和运行)
   - CHEATSHEET.md (常用命令)

2. **深入学习**
   - IMPLEMENTATION_SUMMARY.md (架构和设计)
   - 源代码注释 (详细实现)
   - 运行测试 (功能演示)

3. **自定义扩展**
   - USAGE.md "配置"部分
   - 相关源代码文件
   - 测试文件做参考

4. **后续改进**
   - IMPLEMENTATION_SUMMARY.md "未来扩展方向"
   - 参考.md "技术背景"

---

**最后更新**: 2026-01-29  
**项目状态**: ✅ MVP 完成，生产就绪