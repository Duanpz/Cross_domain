use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
    pub auto_retry: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://127.0.0.1:50051".to_string(),
            timeout_ms: 30000,
            retry_attempts: 3,
            auto_retry: true,
        }
    }
}

impl CliConfig {
    pub fn load_or_create(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(config_path);
        
        // 创建目录
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 如果配置文件不存在，创建默认配置
        if !path.exists() {
            let config = Self::default();
            config.save(config_path)?;
            return Ok(config);
        }
        
        // 加载现有配置
        let content = fs::read_to_string(path)?;
        let config: CliConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}