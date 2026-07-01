use deadpool_redis::{
    Config, ConnectionAddr, ConnectionInfo, Pool, PoolConfig, ProtocolVersion, RedisConnectionInfo, Runtime, Timeouts
};
use std::time::Duration;

use huxley_config::HuxleyConfig;
use crate::HuxleyStoreResult;

pub async fn create_pool(config: &HuxleyConfig) -> HuxleyStoreResult<Pool> {
    let redis_info = RedisConnectionInfo {
        db: config.redis_db,
        username: config.redis_username.clone(),
        password: config.redis_password.clone(),
        protocol: ProtocolVersion::RESP3,
    };

    let connection_addr = match config.redis_use_tls {
        true => ConnectionAddr::TcpTls { host: config.redis_host.clone(), port: config.redis_port, insecure: false },
        false => ConnectionAddr::Tcp(config.redis_host.clone(), config.redis_port),
    };

    let connection_info = ConnectionInfo {
        addr: connection_addr,
        redis: redis_info,
    };

    let mut pool_timeouts = Timeouts::new();
    pool_timeouts.wait = Some(Duration::from_secs(config.redis_wait_timeout));
    pool_timeouts.create = Some(Duration::from_secs(config.redis_create_timeout));
    pool_timeouts.recycle = Some(Duration::from_secs(config.redis_recycle_timeout));

    let pool_config = PoolConfig {
        max_size: config.redis_pool_max_size,
        timeouts: pool_timeouts,
        ..Default::default()
    };

    let options = Config {
        connection: Some(connection_info),
        pool: Some(pool_config),
        ..Default::default()
    };

    let redis_pool = options.create_pool(Some(Runtime::Tokio1))?;

    Ok(redis_pool)
}
