# AI Terminal Inspect

[中文](README.md) | English

An intelligent command-line error diagnostic tool that automatically analyzes command execution failures and provides solutions using AI.

## Features

- 🚀 **Automatic Error Detection**: Automatically captures error output when executing commands
- 🤖 **AI-Powered Analysis**: Calls large language models to analyze error causes
- 📋 **Structured Diagnostic Reports**: Provides error type, root cause analysis, and solutions
- 🎨 **Colored Terminal Output**: Displays diagnostic results with Markdown formatting
- ⚙️ **Flexible Configuration**: Configures API keys and model parameters via environment variables

## System Requirements

- Rust 1.70+ (Edition 2024)
- macOS / Linux / Windows
- Valid LLM API key (supports OpenAI-compatible interfaces)

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd ai-terminal-inspect

# Build the project
cargo build --release

# Add binary to PATH (optional)
cp target/release/ai-terminal-inspect /usr/local/bin/
```

## Configuration

Before using the tool, you need to set the following environment variables:

```bash
# LLM API Key (required)
export API_KEY="your-api-key-here"

# API Base URL (required)
export BASE_URL="https://api.minimaxi.com/v1/text/chatcompletion_v2"

# Model name to use (required)
export MODEL="MiniMax-M2.5"
```

### Supported LLM Providers

This tool supports any service provider compatible with the OpenAI API format, including but not limited to:
- MiniMax
- OpenAI
- Other services with OpenAI-compatible interfaces

## Usage

### Basic Usage

```bash
# Run any command; if it fails, AI analysis will be triggered automatically
ai-terminal-inspect ls /nonexistent/path
ai-terminal-inspect cat missing_file.txt
ai-terminal-inspect ./broken_script.sh
```

### Examples

#### Example 1: Successful Command Execution

```bash
$ ai-terminal-inspect echo "Hello World"
Hello World
```

#### Example 2: Failed Command Execution (Triggers AI Analysis)

```bash
$ ai-terminal-inspect cat nonexistent.txt

Error detected: AI is currently analyzing...

Analysis completed

🚨 Error Type: File Not Found
🔍 Root Cause: The specified file path nonexistent.txt does not exist in the current directory
✅ Solution:
   1. Check if the filename is spelled correctly: ls -la | grep nonexistent
   2. If the file is in another location, use the full path: cat /path/to/file.txt
   3. Create a new file: touch nonexistent.txt
💡 Additional Notes: Use the `ls` command to view the list of files in the current directory
```

## How It Works

1. **Command Parsing**: Retrieves the command to execute from command-line arguments
2. **Command Execution**: Executes the command via shell and captures output
3. **Error Detection**: Checks the command exit status code
4. **AI Analysis**: If failed, sends error information to the LLM
5. **Result Display**: Formats and displays the AI's diagnostic report

## Architecture

```
src/
├── main.rs          # Main program entry point, coordinates all modules
├── command_parse.rs # Command-line argument parsing
├── execute.rs       # Command execution module
├── llm.rs          # LLM API call module
└── config.rs       # Configuration management module
```

### Module Descriptions

- **command_parse**: Parses command-line arguments passed by the user
- **execute**: Executes commands using `sh -c`, captures stdout and stderr
- **llm**: Calls the LLM API, parses responses, and returns analysis results
- **config**: Loads configuration from environment variables (API key, model, URL)

## Tech Stack

- **Rust**: Main programming language
- **tokio**: Async runtime
- **reqwest**: HTTP client
- **serde/serde_json**: JSON serialization/deserialization
- **termimad**: Terminal Markdown rendering
- **crossterm**: Terminal styling control
- **envy**: Environment variable configuration parsing

## Development

### Local Development

```bash
# Clone the repository
git clone <repository-url>
cd ai-terminal-inspect

# Run the project
cargo run -- <your-command>

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Build Release Version

```bash
cargo build --release
```

## Custom System Prompt

You can customize the AI's behavior and output format by modifying the `SYSTEM_PROMPT` constant in `src/llm.rs`.

## Security Notes

⚠️ **Important**:
- Never hardcode API keys in the source code
- Use environment variables to manage sensitive information
- This tool executes arbitrary commands, use with caution
