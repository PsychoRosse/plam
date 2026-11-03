use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = include_str!("../../docs/api/Vec4Error.md")]
pub enum Vec4Error {
    #[error("Vec4 input contains a non-finite value")]
    NonFiniteInput,
    #[error("Vec4 operation produced a non-finite result")]
    InvalidResult,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Cannot normalize a zero-length vector")]
    ZeroLength,
}
