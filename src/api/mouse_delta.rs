use crate::{MouseDeltaError, MousePosition};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseDelta {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MouseDelta {
    pub const MAX_SCROLL_DELTA: f32 = 1_000_000.0;
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseDelta {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Result<Self, MouseDeltaError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(MouseDeltaError::NonFinite);
        }

        if x.abs() > Self::MAX_SCROLL_DELTA || y.abs() > Self::MAX_SCROLL_DELTA {
            return Err(MouseDeltaError::OutOfRange);
        }

        Ok(Self { x, y })
    }
}

impl MouseDelta {
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

    #[inline]
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.x >= -Self::MAX_SCROLL_DELTA
            && self.x <= Self::MAX_SCROLL_DELTA
            && self.y >= -Self::MAX_SCROLL_DELTA
            && self.y <= Self::MAX_SCROLL_DELTA
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, MouseDeltaError> {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }

    #[inline]
    pub fn checked_add_assign(&mut self, rhs: Self) -> Result<(), MouseDeltaError> {
        let result = self.checked_add(rhs)?;
        *self = result;

        Ok(())
    }
}

impl TryFrom<MousePosition> for MouseDelta {
    type Error = MouseDeltaError;

    #[inline]
    fn try_from(pos: MousePosition) -> Result<Self, Self::Error> {
        Self::new(pos.x(), pos.y())
    }
}
