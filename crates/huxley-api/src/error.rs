
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HuxleyApiError {

}

pub type HuxleyApiResult<T> = std::result::Result<T, HuxleyApiError>;
