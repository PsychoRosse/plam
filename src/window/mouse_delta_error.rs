use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseDeltaError {
    #[error("Mouse delta components must be finite")]
    NonFiniteInput,
    #[error("Mouse delta components exceed the permitted range")]
    OutOfRange,
    #[error("Division by zero")]
    DivisionByZero,
}
