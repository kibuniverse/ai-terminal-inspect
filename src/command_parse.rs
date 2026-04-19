use std::env;

pub fn parse() -> Result<Vec<String>, String> {
    let commands = env::args().skip(1).collect::<Vec<String>>();
    Ok(commands)
}
