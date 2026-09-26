pub struct Config {
    pub redis_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        Ok(Self {
            redis_url: env_or("REDIS_URL", "redis://localhost:6379"),
            server_port: env_or("SERVER_PORT", "3000")
                .parse()
                .map_err(|_| ConfigError::Invalid("SERVER PORT"))?,
        })
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

#[derive(Debug)]
pub enum ConfigError {
    Invalid(&'static str),
}
