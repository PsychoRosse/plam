use thiserror::Error;

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
    #[error("Matrix operation produced an invalid result")]
    InvalidResult,
}
