//! 配置文件

use serde::Deserialize;

/// Web 配置
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

/// 应用配置
#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
