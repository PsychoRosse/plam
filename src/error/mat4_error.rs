use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mat4Error {
    #[error("Matrix input contains non-finite values")]
    NonFiniteInput,
    #[error("Matrix is singular and cannot be inverted")]
    SingularMatrix,
    #[error("Invalid perspective projection parameters")]
    InvalidPerspectiveParameters,
    #[error("Invalid quaternion")]
    InvalidQuaternion,
    #[error("Matrix operation produced a non-finite result")]
    InvalidResult,
    #[error("Homogeneous coordinate is zero")]
    ZeroHomogeneousCoordinate,
}
