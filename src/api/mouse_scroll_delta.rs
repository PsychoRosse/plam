use crate::MouseScrollDeltaError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseScrollDelta {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MouseScrollDelta {
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
    pub const MAX_COMPONENT: f32 = 1_000_000.0;
}

impl MouseScrollDelta {
    #[inline]
    #[must_use]
    pub fn new(x: f32, y: f32) -> Result<Self, MouseScrollDeltaError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(MouseScrollDeltaError::NonFinite);
        }

        if x.abs() > Self::MAX_COMPONENT || y.abs() > Self::MAX_COMPONENT {
            return Err(MouseScrollDeltaError::OutOfRange);
        }

        Ok(Self { x, y })
    }
}

impl MouseScrollDelta {
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

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MouseScrollDelta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMouseScrollDelta {
            x: f32,
            y: f32,
        }

        let raw = RawMouseScrollDelta::deserialize(deserializer)?;

        Self::new(raw.x, raw.y).map_err(serde::de::Error::custom)
    }
}
