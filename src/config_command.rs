use crate::cli::ConfigCommand;
use crate::config::Config;

pub fn handle(config_cmd: ConfigCommand) -> Result<(), String> {
    match config_cmd {
        ConfigCommand::Set { key, value } => handle_set(&key, &value),
        ConfigCommand::Show => handle_show(),
        ConfigCommand::Init => handle_init(),
    }
}

fn handle_set(key: &str, value: &str) -> Result<(), String> {
    let mut config = Config::load_from_file().unwrap_or_default();

    match key {
        "api_key" => config.api_key = Some(value.to_string()),
        "model" => config.model = Some(value.to_string()),
        "base_url" => config.base_url = Some(value.to_string()),
        _ => {
            return Err(format!(
                "Unknown config key: {}. Supported keys: api_key, model, base_url",
                key
            ))
        }
    }

    config.save()?;
    println!("Config '{}' has been saved", key);
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

    config.api_key = Some(prompt_with_default(
        "Enter your API Key",
        config.api_key.as_deref(),
    )?);

    config.base_url = Some(prompt_with_default(
        "Enter Base URL",
        config
            .base_url
            .as_deref()
            .or(Some("https://api.minimaxi.com/v1/text/chatcompletion_v2")),
    )?);

    config.model = Some(prompt_with_default(
        "Enter Model Name",
        config.model.as_deref().or(Some("MiniMax-M2.5")),
    )?);

    config.save()?;

    println!("\nConfiguration saved successfully!");
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
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let input = input.trim();

    if input.is_empty() {
        default
            .map(|s| s.to_string())
            .ok_or("Input is required".to_string())
    } else {
        Ok(input.to_string())
    }
}
