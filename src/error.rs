use cosmwasm_std::{Decimal256RangeExceeded, StdError};
use thiserror::Error;

pub type CommonResult<T> = core::result::Result<T, CommonError>;

#[derive(Error, Debug, PartialEq)]
pub enum CommonError {
    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Decimal256RangeExceeded(#[from] Decimal256RangeExceeded),
}
