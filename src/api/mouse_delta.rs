use crate::{MouseDeltaError, MousePosition};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "clama")]
use clama::Event;

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MouseDelta {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MouseDelta {
    pub const MAX_COMPONENT: f32 = 1_000_000.0;
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseDelta {
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
            && self.x >= -Self::MAX_COMPONENT
            && self.x <= Self::MAX_COMPONENT
            && self.y >= -Self::MAX_COMPONENT
            && self.y <= Self::MAX_COMPONENT
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

#[cfg(feature = "serde")]
impl Serialize for MouseDelta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Self::new(self.x, self.y).map_err(serde::ser::Error::custom)?;

        #[derive(Serialize)]
        struct RawMouseDelta {
            x: f32,
            y: f32,
        }

        RawMouseDelta {
            x: self.x,
            y: self.y,
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MouseDelta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMouseDelta {
            x: f32,
            y: f32,
        }

        let raw = RawMouseDelta::deserialize(deserializer)?;
        Self::new(raw.x, raw.y).map_err(serde::de::Error::custom)
    }
}
