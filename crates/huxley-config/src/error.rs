use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuxleyConfigError {
    #[error("Failed to load environment variables")]
    EnvLoad(#[from] envy::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("{0}")]
    ValidationError(String),
}

pub type HuxleyConfigResult<T> = std::result::Result<T, HuxleyConfigError>;
