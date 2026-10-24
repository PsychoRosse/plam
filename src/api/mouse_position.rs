use crate::{MouseDelta, MouseDeltaError};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "clama")]
use clama::Event;

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MousePosition {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MousePosition {
    pub const MAX_COMPONENT: f32 = 1_000_000.0;
}

impl MousePosition {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Result<Self, MouseDeltaError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(MouseDeltaError::NonFinite);
        }

        if x.abs() > Self::MAX_COMPONENT || y.abs() > Self::MAX_COMPONENT {
            return Err(MouseDeltaError::OutOfRange);
        }

        Ok(Self { x, y })
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

    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Result<MouseDelta, MouseDeltaError> {
        MouseDelta::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(feature = "serde")]
impl Serialize for MousePosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::new(self.x, self.y).map_err(serde::ser::Error::custom)?;

        #[derive(Serialize)]
        struct RawMousePosition {
            x: f32,
            y: f32,
        }

        RawMousePosition {
            x: self.x,
            y: self.y,
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MousePosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMousePosition {
            x: f32,
            y: f32,
        }

        let raw = RawMousePosition::deserialize(deserializer)?;

        Self::new(raw.x, raw.y).map_err(serde::de::Error::custom)
    }
}
