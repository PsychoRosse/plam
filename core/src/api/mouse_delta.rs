use crate::MousePosition;

use std::ops::AddAssign;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseDelta {
    x: f32,
    y: f32,
}

impl MouseDelta {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub fn try_new(x: f32, y: f32) -> Option<Self> {
        (x.is_finite() && y.is_finite()).then_some(Self::new(x, y))
    }
}

impl MouseDelta {
    #[inline]
    #[must_use]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    pub fn y(&self) -> f32 {
        self.y
    }

    #[inline]
    #[must_use]
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self::new(x, y)
    }

    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl AddAssign for MouseDelta {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<MousePosition> for MouseDelta {
    fn from(pos: MousePosition) -> Self {
        Self::new(pos.x(), pos.y())
    }
}
