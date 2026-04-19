
use crossterm::style::Color;
use termimad::MadSkin;

mod command_parse;
mod execute;
mod llm;

mod config;
mod config_command;

#[tokio::main]
async fn main() {
    let commands = command_parse::parse().unwrap_or_else(|e| {
        eprintln!("Error parsing arguments: {}", e);
        std::process::exit(1);
    });

    if commands.is_empty() {
        eprintln!("No command provided. Usage: ati <command>");
        std::process::exit(1);
    }

    // 检查是否是 config 子命令
    if !commands.is_empty() && commands[0] == "config" {
        if let Err(e) = config_command::handle_config_command(&commands[1..]) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // 正常执行命令
    let exec_result = execute::run_shell(&commands.join(" "));
    match exec_result {
        Ok(output) => println!("{}", output),
        Err(error) => {
            let config = match config::Config::init() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error initializing config: {}", e);
                    return;
                }
            };

            let error_message = error.to_string();
            let mut skin = MadSkin::default();
            skin.paragraph.set_fg(Color::Red);
            skin.print_inline("Error detected:");
            skin = MadSkin::default();
            skin.print_text(" AI is currently analyzing...");

            match llm::call_llm(&error_message, &config).await {
                Ok(llm_response) => {
                    skin.paragraph.set_fg(Color::Green);
                    skin.print_text("Analysis completed");
                    skin = MadSkin::default();
                    skin.print_text(&llm_response);
                }
                Err(e) => eprintln!("AI analysis failed: {}", e),
            }
        }
    }
}
