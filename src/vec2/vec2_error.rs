use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec2Error {
    #[error("Vector contains non-finite values")]
    NonFiniteInput,
    #[error("Vector has zero length and cannot be normalized")]
    ZeroLength,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Vector operation produced a non-finite result")]
    InvalidResult,
}
