use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuatError {
    #[error("Quaternion input contains a non-finite value")]
    NonFiniteInput,
    #[error("Quaternion operation produced a non-finite result")]
    InvalidResult,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Cannot normalize or invert a zero-length quaternion")]
    ZeroLength,
    #[error("Axis must be finite and non-zero")]
    InvalidAxis,
}
