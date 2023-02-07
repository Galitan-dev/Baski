use std::{fmt::Display, fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProfilesConfig {
    global: Option<ConfigProfile>,
    development: Option<ConfigProfile>,
    production: Option<ConfigProfile>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigProfile {
    hostname: Option<String>,
    port: Option<u16>,
    live_reloading: Option<bool>,
    log_level: Option<LogLevel>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub environment: Environment,
    pub hostname: String,
    pub port: u16,
    pub live_reloading: bool,
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "error")]
    Error,
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "development" | "dev" | "debug" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(format!("Incorrect APP_ENV: {value}")),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Environment::Development => "development",
            Environment::Production => "production",
        })
    }
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let profiles: ProfilesConfig = toml::from_str(&content).unwrap();
        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or("development".to_owned())
            .try_into()
            .unwrap();

        let global_config = profiles.global.unwrap_or(ConfigProfile::default());
        let profile_config = match environment {
            Environment::Development => profiles.development,
            Environment::Production => profiles.production,
        }
        .unwrap_or(ConfigProfile::default());

        Self {
            environment,
            hostname: profile_config
                .hostname
                .or(global_config.hostname)
                .expect(&format!("hostname not specified for {environment}")),
            port: profile_config
                .port
                .or(global_config.port)
                .expect(&format!("port not specified for {environment}")),
            live_reloading: profile_config
                .live_reloading
                .or(global_config.live_reloading)
                .expect(&format!("live_reloading not specified for {environment}")),
            log_level: profile_config
                .log_level
                .or(global_config.log_level)
                .expect(&format!("log_level not specified for {environment}")),
        }
    }
}
