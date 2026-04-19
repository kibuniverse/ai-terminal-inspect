use crate::config::Config;

/// 配置子命令
#[derive(Debug)]
pub enum ConfigCommand {
    /// 设置配置项
    Set {
        key: String,
        value: String,
    },
    /// 显示当前配置
    Show,
    /// 初始化配置（交互式）
    Init,
}

pub fn handle_config_command(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let command = &args[0];

    match command.as_str() {
        "set" => handle_set(&args[1..]),
        "show" => handle_show(),
        "init" => handle_init(),
        _ => {
            eprintln!("Unknown config command: {}", command);
            print_usage();
            Err("Invalid config command".to_string())
        }
    }
}

fn handle_set(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: ati config set <key> <value>\nSupported keys: api_key, model, base_url".to_string());
    }

    let key = &args[0];
    let value = &args[1];

    // 加载现有配置
    let mut config = Config::load_from_file().unwrap_or_default();

    // 更新对应的配置项
    match key.as_str() {
        "api_key" => config.api_key = Some(value.clone()),
        "model" => config.model = Some(value.clone()),
        "base_url" => config.base_url = Some(value.clone()),
        _ => return Err(format!("Unknown config key: {}. Supported keys: api_key, model, base_url", key)),
    }

    // 保存配置
    config.save()?;
    println!("✓ Config '{}' has been saved", key);
    
    Ok(())
}

fn handle_show() -> Result<(), String> {
    let config = Config::load_from_file().unwrap_or_default();
    config.show();
    Ok(())
}

fn handle_init() -> Result<(), String> {
    println!("AI Terminal Inspect - Configuration Setup\n");
    
    let mut config = Config::load_from_file().unwrap_or_default();

    // 交互式设置
    config.api_key = Some(prompt_with_default(
        "Enter your API Key",
        config.api_key.as_deref(),
    )?);

    config.base_url = Some(prompt_with_default(
        "Enter Base URL",
        config.base_url.as_deref().or(Some("https://api.minimaxi.com/v1/text/chatcompletion_v2")),
    )?);

    config.model = Some(prompt_with_default(
        "Enter Model Name",
        config.model.as_deref().or(Some("MiniMax-M2.5")),
    )?);

    // 保存配置
    config.save()?;
    
    println!("\n✓ Configuration saved successfully!");
    println!("You can now use 'ati <command>' to run commands with AI inspection.");
    
    Ok(())
}

fn prompt_with_default(message: &str, default: Option<&str>) -> Result<String, String> {
    use std::io::{self, Write};

    let prompt = match default {
        Some(val) => format!("{} [{}]: ", message, val),
        None => format!("{}: ", message),
    };

    print!("{}", prompt);
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let input = input.trim();
    
    if input.is_empty() {
        default.map(|s| s.to_string())
            .ok_or("Input is required".to_string())
    } else {
        Ok(input.to_string())
    }
}

fn print_usage() {
    println!("Configuration Management");
    println!();
    println!("Usage:");
    println!("  ati config set <key> <value>  Set a config value");
    println!("  ati config show               Show current config");
    println!("  ati config init               Interactive config setup");
    println!();
    println!("Config keys:");
    println!("  api_key     Your LLM API key");
    println!("  base_url    API endpoint URL");
    println!("  model       Model name to use");
    println!();
    println!("Examples:");
    println!("  ati config set api_key your-api-key-here");
    println!("  ati config set base_url https://api.openai.com/v1");
    println!("  ati config set model gpt-4");
}
