use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
}

impl Config {
    /// 获取配置文件路径
    fn config_path() -> Option<PathBuf> {
        use directories::BaseDirs;
        let base = BaseDirs::new()?;
        let config_dir = base.home_dir().join(".config/ai-terminal-inspect");
        fs::create_dir_all(&config_dir).ok();
        Some(config_dir.join("config.toml"))
    }

    /// 从配置文件加载配置
    pub fn load_from_file() -> Option<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path).ok()?;
            toml::from_str(&content).ok()
        } else {
            None
        }
    }

    /// 从环境变量加载配置
    fn load_from_env() -> Self {
        Config {
            api_key: std::env::var("API_KEY").ok(),
            model: std::env::var("MODEL").ok(),
            base_url: std::env::var("BASE_URL").ok(),
        }
    }

    /// 初始化配置（优先级：环境变量 > 配置文件）
    pub fn init() -> Result<Self, String> {
        let file_config = Self::load_from_file().unwrap_or_default();
        let env_config = Self::load_from_env();

        // 环境变量优先
        let config = Config {
            api_key: env_config.api_key.or(file_config.api_key),
            model: env_config.model.or(file_config.model),
            base_url: env_config.base_url.or(file_config.base_url),
        };

        // 验证必填字段
        if config.api_key.is_none() {
            return Err("API_KEY is required. Run 'ati config set' to configure.".to_string());
        }
        if config.model.is_none() {
            return Err("MODEL is required. Run 'ati config set' to configure.".to_string());
        }
        if config.base_url.is_none() {
            return Err("BASE_URL is required. Run 'ati config set' to configure.".to_string());
        }

        Ok(config)
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path().ok_or("Failed to determine config directory".to_string())?;

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&path, toml_string).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// 显示当前配置（隐藏敏感信息）
    pub fn show(&self) {
        println!("Configuration file: {:?}", Self::config_path());
        println!("\nCurrent settings:");

        if let Some(ref api_key) = self.api_key {
            let masked = if api_key.len() > 8 {
                format!("{}...{}", &api_key[..4], &api_key[api_key.len() - 4..])
            } else {
                "****".to_string()
            };
            println!("  api_key = {}", masked);
        } else {
            println!("  api_key = (not set)");
        }

        if let Some(ref model) = self.model {
            println!("  model = {}", model);
        } else {
            println!("  model = (not set)");
        }

        if let Some(ref base_url) = self.base_url {
            println!("  base_url = {}", base_url);
        } else {
            println!("  base_url = (not set)");
        }
    }
}
