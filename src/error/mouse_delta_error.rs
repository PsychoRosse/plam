use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = include_str!("../../docs/api/MouseDeltaError.md")]
pub enum MouseDeltaError {
    #[error("Mouse delta components must be finite")]
    NonFinite,
    #[error("Mouse delta components exceed the permitted range")]
    OutOfRange,
}
