use config::{Config, ConfigError, File};

use crate::{
    errors::{failed_current_dir, failed_env_parse},
    settings::Request,
};

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
    pub port: u16,
    pub servers: Vec<String>,
}
impl CdnSettings {
    // TODO: i need to program it so that it pings the servers
    //  and update it based on those who are up
    pub fn check_up_servers(&mut self) {
        let settings = get_config().unwrap();
        self.servers = settings.cdn.servers;
    }
    pub async fn health_check(&self) -> Vec<&str> {
        let mut up_servers = vec![];
        let mut req = Request::new(self.port);
        for server in &self.servers {
            req.set_addr(&server);
            if super::health_check::check(&req).await {
                up_servers.push(server.as_str());
            }
        }
        up_servers
    }
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
