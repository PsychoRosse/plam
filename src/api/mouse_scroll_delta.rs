use crate::MouseScrollDeltaError;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "clama")]
use clama::Event;

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseScrollDelta {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MouseScrollDelta {
    pub const DEFAULT_SCROLL_MULTIPLIER: f32 = 32.0;
    pub const MAX_COMPONENT: f32 = 1_000_000.0;
}

impl MouseScrollDelta {
    #[inline]
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
impl Serialize for MouseScrollDelta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::new(self.x, self.y).map_err(serde::ser::Error::custom)?;

        #[derive(Serialize)]
        struct RawMouseScrollDelta {
            x: f32,
            y: f32,
        }

        RawMouseScrollDelta {
            x: self.x,
            y: self.y,
        }
        .serialize(serializer)
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
