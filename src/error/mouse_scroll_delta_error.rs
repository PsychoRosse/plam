use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = include_str!("../../docs/api/MouseScrollDeltaError.md")]
pub enum MouseScrollDeltaError {
    #[error("Mouse scroll delta components must be finite")]
    NonFinite,
    #[error("Mouse scroll delta components exceed the permitted range")]
    OutOfRange,
}
