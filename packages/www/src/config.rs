use bdk::prelude::*;
use tracing::Level;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub log_level: Level,
    pub api_endpoint: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            log_level: match option_env!("RUST_LOG") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
            api_endpoint: option_env!("API_ENDPOINT").unwrap_or("https://home-api.dev.biyard.co"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
