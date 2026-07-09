mod error;

use serde::Deserialize;

pub use error::{HuxleyConfigError, HuxleyConfigResult};

#[derive(Debug, Clone, Deserialize)]
pub struct HuxleyConfig {
    #[serde(rename = "HUXLEY_LOG_LEVEL")]
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(rename = "HUXLEY_ENVIRONMENT")]
    #[serde(default = "default_environment")]
    pub environment: String,

    #[serde(rename = "HUXLEY_URL")]
    #[serde(default = "default_url")]
    pub ui_url: String,
    #[serde(rename = "HUXLEY_PROTOCOL")]
    #[serde(default = "default_protocol")]
    pub ui_protocol: String,
    #[serde(rename = "HUXLEY_PORT")]
    #[serde(default = "default_port")]
    pub ui_port: u16,

    #[serde(rename = "HUXLEY_DB_HOST")]
    #[serde(default = "default_db_host")]
    pub db_host: String,
    #[serde(rename = "HUXLEY_DB_PORT")]
    #[serde(default = "default_db_port")]
    pub db_port: u16,
    #[serde(rename = "HUXLEY_DB_USERNAME")]
    pub db_username: String,
    #[serde(rename = "HUXLEY_DB_PASSWORD")]
    pub db_password: String,
    #[serde(rename = "HUXLEY_DB_DATABASE")]
    #[serde(default = "default_db_database")]
    pub db_database: String,
    #[serde(rename = "HUXLEY_DB_USE_TLS")]
    #[serde(default = "default_db_use_tls")]
    pub db_use_tls: bool,
    #[serde(rename = "HUXLEY_DB_MIN_CONNECTIONS")]
    #[serde(default = "default_db_min_connections")]
    pub db_min_connections: u32,
    #[serde(rename = "HUXLEY_DB_MAX_CONNECTIONS")]
    #[serde(default = "default_db_max_connections")]
    pub db_max_connections: u32,
    #[serde(rename = "HUXLEY_DB_IDLE_TIMEOUT")]
    #[serde(default = "default_db_idle_timeout")]
    pub db_idle_timeout: u64,
    #[serde(rename = "HUXLEY_DB_POOL_ACQUIRE_TIMEOUT")]
    #[serde(default = "default_db_pool_acquire_timeout")]
    pub db_pool_acquire_timeout: u64,

    #[serde(rename = "HUXLEY_CACHE_HOST")]
    #[serde(default = "default_cache_host")]
    pub redis_host: String,
    #[serde(rename = "HUXLEY_CACHE_PORT")]
    #[serde(default = "default_cache_port")]
    pub redis_port: u16,
    #[serde(rename = "HUXLEY_CACHE_USERNAME")]
    pub redis_username: Option<String>,
    #[serde(rename = "HUXLEY_CACHE_PASSWORD")]
    pub redis_password: Option<String>,
    #[serde(rename = "HUXLEY_CACHE_DB")]
    #[serde(default = "default_cache_db")]
    pub redis_db: i64,
    #[serde(rename = "HUXLEY_CACHE_POOL_MAX_SIZE")]
    #[serde(default = "default_cache_pool_max_size")]
    pub redis_pool_max_size: usize,
    #[serde(rename = "HUXLEY_CACHE_USE_TLS")]
    #[serde(default = "default_cache_use_tls")]
    pub redis_use_tls: bool,
    #[serde(rename = "HUXLEY_CACHE_TIMEOUT")]
    #[serde(default = "default_cache_timeout")]
    pub redis_timeout: u64,
    #[serde(rename = "HUXLEY_CACHE_WAIT_TIMEOUT")]
    #[serde(default = "default_cache_wait_timeout")]
    pub redis_wait_timeout: u64,
    #[serde(rename = "HUXLEY_CACHE_CREATE_TIMEOUT")]
    #[serde(default = "default_cache_create_timeout")]
    pub redis_create_timeout: u64,
    #[serde(rename = "HUXLEY_CACHE_RECYCLE_TIMEOUT")]
    #[serde(default = "default_cache_recycle_timeout")]
    pub redis_recycle_timeout: u64,
}

impl HuxleyConfig {
    pub fn from_env() -> HuxleyConfigResult<HuxleyConfig> {
        dotenvy::dotenv().ok();
        let config: HuxleyConfig = envy::from_env::<HuxleyConfig>()?;

        Ok(config)
    }

    pub fn is_production(self) -> bool {
        self.environment == "production"
    }
}

fn default_log_level() -> String {
    "info".into()
}

fn default_environment() -> String {
    "production".into()
}

fn default_url() -> String {
    "localhost".into()
}

fn default_protocol() -> String {
    "http".into()
}

fn default_port() -> u16 {
    8080
}

fn default_db_host() -> String {
    "localhost".into()
}

fn default_db_port() -> u16 {
    5432
}

fn default_db_database() -> String {
    "huxley".into()
}

fn default_db_use_tls() -> bool {
    true
}

fn default_db_min_connections() -> u32 {
    5
}

fn default_db_max_connections() -> u32 {
    20
}

fn default_db_idle_timeout() -> u64 {
    600
}

fn default_db_pool_acquire_timeout() -> u64 {
    5
}

fn default_cache_host() -> String {
    "localhost".into()
}

fn default_cache_port() -> u16 {
    6379
}

fn default_cache_db() -> i64 {
    0
}

fn default_cache_pool_max_size() -> usize {
    16
}

fn default_cache_use_tls() -> bool {
    true
}

fn default_cache_timeout() -> u64 {
    10
}

fn default_cache_wait_timeout() -> u64 {
    2
}

fn default_cache_create_timeout() -> u64 {
    3
}

fn default_cache_recycle_timeout() -> u64 {
    1
}
