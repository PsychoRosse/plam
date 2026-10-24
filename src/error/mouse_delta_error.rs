use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseDeltaError {
    #[error("Mouse delta components must be finite")]
    NonFinite,
    #[error("Mouse delta components exceed the permitted range")]
    OutOfRange,
}
