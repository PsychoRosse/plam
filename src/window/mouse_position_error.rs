use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MousePositionError {
    #[error("Mouse position components must be finite")]
    NonFiniteInput,
    #[error("Mouse position components exceed the permitted range")]
    OutOfRange,
}
