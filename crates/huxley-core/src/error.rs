use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuxleyCoreError {

}

pub type HuxleyCoreResult<T> = std::result::Result<T, HuxleyCoreError>;
