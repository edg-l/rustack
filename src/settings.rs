use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub url: String,
    pub secure: bool,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Smtp {
    pub domain: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize)]
pub struct Files {
    pub static_dir: String,
    pub templates_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub http: Http,
    pub smtp: Smtp,
    pub files: Files,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();
        let mut s = Config::new();
        s.merge(File::with_name("conf/default"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("conf/{}", env)).required(false))?;

        // APP_DEBUG would set debug key
        s.merge(Environment::with_prefix("app"))?;

        s.try_into()
    }
}
