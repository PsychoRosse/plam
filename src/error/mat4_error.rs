use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[doc = include_str!("../../docs/api/Mat4Error.md")]
pub enum Mat4Error {
    #[error("Matrix input contains non-finite values")]
    NonFiniteInput,
    #[error("Matrix is singular and cannot be inverted")]
    SingularMatrix,
    #[error("Invalid perspective projection parameters")]
    InvalidPerspectiveParameters,
    #[error("Invalid orthographic projection parameters")]
    InvalidOrthographicParameters,
    #[error(
        "Invalid look-at parameters: eye and center coincide, or up is parallel to the view direction"
    )]
    InvalidLookAtParameters,
    #[error("Invalid quaternion")]
    InvalidQuaternion,
    #[error("Matrix operation produced a non-finite result")]
    InvalidResult,
    #[error("Homogeneous coordinate is zero")]
    ZeroHomogeneousCoordinate,
}
