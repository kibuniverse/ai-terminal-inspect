# AI Terminal Inspect

[English](README.en.md) | 中文

一个智能命令行错误诊断工具，使用 AI 自动分析命令执行失败的原因并提供解决方案。

## 功能特性

- 🚀 **自动错误检测**：执行命令时自动捕获错误输出
- 🤖 **AI 智能分析**：调用大语言模型分析错误原因
- 📋 **结构化诊断报告**：提供错误类型、根源分析和解决方案
- 🎨 **彩色终端输出**：使用 Markdown 格式化显示诊断结果
- ⚙️ **灵活配置**：通过环境变量配置 API 密钥和模型参数

## 系统要求

- Rust 1.70+ (Edition 2024)
- macOS / Linux / Windows
- 有效的 LLM API 密钥（支持 OpenAI 兼容接口）

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone <repository-url>
cd ai-terminal-inspect

# 编译项目
cargo build --release

# 将二进制文件添加到 PATH（可选）
cp target/release/ai-terminal-inspect /usr/local/bin/
```

## 配置

在使用前，需要设置以下环境变量：

```bash
# LLM API 密钥（必需）
export API_KEY="your-api-key-here"

# API 基础 URL（必需）, 任何符合OpenAI 兼容接口均可
export BASE_URL="https://your-api-host/v1/chat/completions"

# 使用的模型名称（必需）
export MODEL="gpt-4"
```

### 支持的 LLM 提供商

本工具支持任何兼容 OpenAI API 格式的服务商，包括但不限于：
- MiniMax
- ZAI
- OpenAI
- DeepSeek
- 其他兼容 OpenAI 接口的服务

## 使用方法

### 基本用法

```bash
# 运行任意命令，如果出错会自动触发 AI 分析
ai-terminal-inspect ls /nonexistent/path
ai-terminal-inspect cat missing_file.txt
ai-terminal-inspect ./broken_script.sh
```

### 示例

#### 示例 1：命令执行成功

```bash
$ ai-terminal-inspect echo "Hello World"
Hello World
```

#### 示例 2：命令执行失败（触发 AI 分析）

```bash
$ ai-terminal-inspect cat nonexistent.txt

Error detected: AI is currently analyzing...

Analysis completed

🚨 错误类型：文件不存在
🔍 问题根源：指定的文件路径 nonexistent.txt 在当前目录中不存在
✅ 解决方案：
   1. 检查文件名是否拼写正确：ls -la | grep nonexistent
   2. 如果文件在其他位置，使用完整路径：cat /path/to/file.txt
   3. 创建新文件：touch nonexistent.txt
💡 补充说明：使用 `ls` 命令查看当前目录下的文件列表
```

## 工作原理

1. **命令解析**：从命令行参数获取要执行的命令
2. **命令执行**：通过 shell 执行命令并捕获输出
3. **错误检测**：检查命令退出状态码
4. **AI 分析**：如果失败，将错误信息发送给 LLM
5. **结果展示**：格式化显示 AI 的诊断报告

## 架构说明

```
src/
├── main.rs          # 主程序入口，协调各模块工作
├── command_parse.rs # 命令行参数解析
├── execute.rs       # 命令执行模块
├── llm.rs          # LLM API 调用模块
└── config.rs       # 配置管理模块
```

### 模块说明

- **command_parse**: 解析用户传入的命令行参数
- **execute**: 使用 `sh -c` 执行命令，捕获 stdout 和 stderr
- **llm**: 调用大语言模型 API，解析响应并返回分析结果
- **config**: 从环境变量加载配置（API 密钥、模型、URL）

## 技术栈

- **Rust**: 主要编程语言
- **tokio**: 异步运行时
- **reqwest**: HTTP 客户端
- **serde/serde_json**: JSON 序列化/反序列化
- **termimad**: 终端 Markdown 渲染
- **crossterm**: 终端样式控制
- **envy**: 环境变量配置解析

## 开发

### 本地开发

```bash
# 克隆仓库
git clone <repository-url>
cd ai-terminal-inspect

# 运行项目
cargo run -- <your-command>

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 构建发布版本

```bash
cargo build --release
```

## 自定义系统提示词

你可以修改 `src/llm.rs` 中的 `SYSTEM_PROMPT` 常量来定制 AI 的行为和输出格式。

## 安全注意事项

⚠️ **重要提示**：
- 不要在代码中硬编码 API 密钥
- 使用环境变量管理敏感信息
- 本工具会执行任意命令，请谨慎使用
