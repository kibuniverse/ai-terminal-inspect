
use crossterm::style::Color;
use termimad::MadSkin;
use tokio;

mod command_parse;
mod execute;
mod llm;

mod config;
mod config_command;

#[tokio::main]
async fn main() {
    let commands = command_parse::parse().unwrap();
    // 检查是否是 config 子命令
    if commands.len() >= 1 && commands[0] == "config" {
        if let Err(e) = config_command::handle_config_command(&commands[1..]) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // 正常执行命令
    let exec_result = execute::run_shell(&commands.join(" "));
    if let Ok(output) = exec_result {
        println!("{}", output);
    } else {
        let config = config::Config::init();
        if let Err(config_error) = config {
            eprintln!("Error initializing config: {}", config_error);
            return;
        };
        let error_message = exec_result.err().unwrap().to_string();
        let mut skin = MadSkin::default();
        skin.paragraph.set_fg(Color::Red);
        skin.print_inline("Error detected:");
        skin = MadSkin::default();
        skin.print_text(" AI is currently analyzing...");
        let llm_response = llm::call_llm(&error_message, &config.unwrap())
            .await
            .unwrap();
        skin.paragraph.set_fg(Color::Green);
        skin.print_text("Analysis completed");
        skin = MadSkin::default();
        skin.print_text(&llm_response);
    }
}
