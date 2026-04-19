mod cli;
mod config;
mod config_command;
mod display;
mod execute;
mod llm;

#[tokio::main]
async fn main() {
    let command = cli::parse_args().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    match command {
        cli::Command::Config { args } => {
            let config_cmd = cli::parse_config_command(&args).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            if let Err(e) = config_command::handle(config_cmd) {
                eprintln!("Error: {}", e); 
                std::process::exit(1);
            }
        }
        cli::Command::Run { command } => {
            run_and_analyze(&command).await;
        }
    }
}

async fn run_and_analyze(command: &str) {
    match execute::run_shell(command) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            let config = match config::Config::init() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error initializing config: {}", e);
                    return;
                }
            };
            display::print_error_detected();
            match llm::call_llm(&error.to_string(), &config).await {
                Ok(response) => display::print_analysis(&response),
                Err(e) => eprintln!("AI analysis failed: {}", e),
            }
        }
    }
}
