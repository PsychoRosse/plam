use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
