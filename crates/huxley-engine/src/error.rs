use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuxleyEngineError {

}

pub type HuxleyEngineResult<T> = std::result::Result<T, HuxleyEngineError>;
