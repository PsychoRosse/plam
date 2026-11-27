use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec2Error {
    #[error("Vec2 input contains a non-finite value")]
    NonFiniteInput,
    #[error("Vec2 operation produced a non-finite result")]
    InvalidResult,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Cannot normalize a zero-length vector")]
    ZeroLength,
    #[error("Invalid clamp range")]
    InvalidRange,
}
