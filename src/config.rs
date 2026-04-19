use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Config {
    pub fn init() -> Result<Self, envy::Error> {
        let config = envy::from_env::<Config>()?;
        Ok(Config {
            ..config    
        })
    }
}
