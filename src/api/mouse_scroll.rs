use crate::MouseScrollDelta;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "clama")]
use clama::Event;

#[cfg_attr(feature = "clama", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MouseScroll {
    Lines(MouseScrollDelta),
    Pixels(MouseScrollDelta),
}
