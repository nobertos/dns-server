use config::{Config, ConfigError, File};

use crate::errors::{failed_current_dir, failed_env_parse};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub cdn: CdnSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplicationSettings {
    pub is_load_balancer: bool,
    pub port: u16,
    pub host: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct CdnSettings {
    pub hostname: String,
    pub connections_path: String,
    pub up_servers: Vec<String>,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            _ => Err(failed_env_parse(&value)),
        }
    }
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect(failed_current_dir());
    let config_dir = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .unwrap();

    let settings = Config::builder()
        .add_source(File::from(config_dir.join("base")).required(true))
        .add_source(File::from(config_dir.join(environment.as_str())).required(true))
        .build()?;

    settings.try_deserialize::<Settings>()
}
