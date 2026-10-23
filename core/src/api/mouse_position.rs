use crate::MouseDelta;

use std::ops::Sub;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MousePosition {
    x: f32,
    y: f32,
}

impl MousePosition {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MousePosition {
    #[inline]
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }
}

impl Sub for MousePosition {
    type Output = MouseDelta;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        MouseDelta::new(self.x - rhs.x, self.y - rhs.y)
    }
}
