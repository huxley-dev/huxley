use sqlx::{
    postgres::{PgPoolOptions, PgConnectOptions, PgSslMode},
    PgPool,
    migrate,
};
use std::time::Duration;

use huxley_config::HuxleyConfig;
use crate::HuxleyStoreResult;

pub async fn create_pool(config: &HuxleyConfig) -> HuxleyStoreResult<PgPool> {
    let mut options: PgConnectOptions = PgConnectOptions::new()
        .host(&config.db_host.clone())
        .port(config.db_port)
        .database(&config.db_database.clone())
        .username(&config.db_username.clone())
        .password(&config.db_password.clone());

    if config.db_use_tls {
        options = options.ssl_mode(PgSslMode::Require);
    }

    let pool = PgPoolOptions::new()
        .min_connections(config.db_min_connections)
        .max_connections(config.db_max_connections)
        .idle_timeout(Duration::from_secs(config.db_idle_timeout))
        .acquire_timeout(Duration::from_secs(config.db_pool_acquire_timeout))
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(db_pool: &PgPool) -> HuxleyStoreResult<()> {
    migrate!("./migrations")
        .run(db_pool)
        .await?;

    Ok(())
}
