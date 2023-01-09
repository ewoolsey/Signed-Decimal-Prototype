use cosmwasm_std::{ConversionOverflowError, Decimal256RangeExceeded, StdError};
use thiserror::Error;

pub type CommonResult<T> = core::result::Result<T, CommonError>;

#[derive(Error, Debug, PartialEq)]
pub enum CommonError {
    #[error("{0}")]
    Error(String),

    #[error("{0}")]
    Generic(String),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    Decimal256RangeExceeded(#[from] Decimal256RangeExceeded),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Only one tx is allowed per block")]
    MultipleTx {},

    #[error("Missing Cw20HookMg")]
    MissingHookMsg {},
}
