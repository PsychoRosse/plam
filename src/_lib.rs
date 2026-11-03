#![doc = include_str!("../docs/api/lib.md")]

mod api;
mod error;

#[doc = include_str!("../docs/api/prelude.md")]
pub mod prelude;

pub use {api::*, error::*};
