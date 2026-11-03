use crate::MouseScrollDelta;

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "clama", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc = include_str!("../../docs/api/MouseScroll.md")]
pub enum MouseScroll {
    Lines(MouseScrollDelta),
    Pixels(MouseScrollDelta),
}
