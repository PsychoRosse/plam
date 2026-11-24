use crate::{MouseScrollDelta, MouseScrollDeltaError};

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "clama", derive(Event))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScroll {
    Lines(MouseScrollDelta),
    Pixels(MouseScrollDelta),
}

impl MouseScroll {
    #[inline]
    #[must_use]
    pub const fn delta(self) -> MouseScrollDelta {
        match self {
            Self::Lines(delta) | Self::Pixels(delta) => delta,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_lines(self) -> bool {
        match self {
            Self::Lines(_) => true,
            Self::Pixels(_) => false,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_pixels(self) -> bool {
        match self {
            Self::Pixels(_) => true,
            Self::Lines(_) => false,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.delta().is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.delta().is_zero()
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        self.delta().length()
    }

    #[inline]
    #[must_use]
    pub fn max_component(self) -> f32 {
        self.delta().max_component()
    }

    #[inline]
    pub fn scaled(self, multiplier: f32) -> Result<Self, MouseScrollDeltaError> {
        match self {
            Self::Lines(delta) => Ok(Self::Lines(delta.scaled(multiplier)?)),
            Self::Pixels(delta) => Ok(Self::Pixels(delta.scaled(multiplier)?)),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_approx_eq(self, rhs: Self, epsilon: f32) -> bool {
        match (self, rhs) {
            (Self::Lines(a), Self::Lines(b)) | (Self::Pixels(a), Self::Pixels(b)) => {
                a.is_approx_eq(b, epsilon)
            }
            _ => false,
        }
    }
}
