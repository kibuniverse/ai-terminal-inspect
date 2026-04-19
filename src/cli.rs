use std::env;

pub enum Command {
    Config { args: Vec<String> },
    Run { command: String },
}

#[derive(Debug)]
pub enum ConfigCommand {
    Set { key: String, value: String },
    Show,
    Init,
}

pub fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        return Err("No command provided. Usage: ati <command>".to_string());
    }

    if args[0] == "config" {
        Ok(Command::Config {
            args: args[1..].to_vec(),
        })
    } else {
        Ok(Command::Run {
            command: args.join(" "),
        })
    }
}

pub fn parse_config_command(args: &[String]) -> Result<ConfigCommand, String> {
    if args.is_empty() {
        print_config_usage();
        return Err("No config subcommand provided".to_string());
    }

    match args[0].as_str() {
        "set" => {
            if args.len() < 3 {
                return Err(
                    "Usage: ati config set <key> <value>\nSupported keys: api_key, model, base_url"
                        .to_string(),
                );
            }
            Ok(ConfigCommand::Set {
                key: args[1].clone(),
                value: args[2].clone(),
            })
        }
        "show" => Ok(ConfigCommand::Show),
        "init" => Ok(ConfigCommand::Init),
        other => {
            print_config_usage();
            Err(format!("Unknown config command: {}", other))
        }
    }
}

fn print_config_usage() {
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
