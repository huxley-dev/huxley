use deadpool_redis::{CreatePoolError, PoolError};
use redis::RedisError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuxleyStoreError {
    #[error("Database error occurred: {0}")]
    Db(#[from] sqlx::Error),

    #[error("Database migration error occurred: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("Redis connection pool creation error: {0}")]
    PoolCreate(#[from] CreatePoolError),

    #[error("Redis connection pool error: {0}")]
    Pool(#[from] PoolError),

    #[error("Reddis backend error: {0}")]
    Redis(#[from] RedisError),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type HuxleyStoreResult<T> = std::result::Result<T, HuxleyStoreError>;
