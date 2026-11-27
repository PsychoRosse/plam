use crate::MouseScrollDeltaError;

#[cfg(feature = "clama")]
use clama::Event;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(feature = "clama", derive(Event))]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct MouseScrollDelta {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl MouseScrollDelta {
    pub const DEFAULT_MULTIPLIER: f32 = 32.0;
    pub const MAX_COMPONENT: f32 = 1_000_000.0;
}

impl MouseScrollDelta {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Result<Self, MouseScrollDeltaError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(MouseScrollDeltaError::NonFiniteInput);
        }

        if x.abs() > Self::MAX_COMPONENT || y.abs() > Self::MAX_COMPONENT {
            return Err(MouseScrollDeltaError::OutOfRange);
        }

        Ok(Self { x, y })
    }

    #[inline]
    #[must_use]
    pub const fn x(self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    pub const fn y(self) -> f32 {
        self.y
    }

    #[inline]
    pub fn scaled(self, multiplier: f32) -> Result<Self, MouseScrollDeltaError> {
        if !multiplier.is_finite() {
            return Err(MouseScrollDeltaError::NonFiniteInput);
        }

        Self::new(self.x * multiplier, self.y * multiplier)
    }

    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    #[inline]
    pub const fn from_array(values: [f32; 2]) -> Result<Self, MouseScrollDeltaError> {
        Self::new(values[0], values[1])
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn splat(value: f32) -> Result<Self, MouseScrollDeltaError> {
        Self::new(value, value)
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    #[inline]
    #[must_use]
    pub fn max_component(self) -> f32 {
        self.x.abs().max(self.y.abs())
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, MouseScrollDeltaError> {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }

    #[inline]
    pub fn checked_add_assign(&mut self, rhs: Self) -> Result<(), MouseScrollDeltaError> {
        *self = self.checked_add(rhs)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, MouseScrollDeltaError> {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }

    #[inline]
    pub fn checked_sub_assign(&mut self, rhs: Self) -> Result<(), MouseScrollDeltaError> {
        *self = self.checked_sub(rhs)?;
        Ok(())
    }

    #[inline]
    pub fn is_approx_eq(self, rhs: Self, epsilon: f32) -> bool {
        if !self.is_finite() || !rhs.is_finite() || !epsilon.is_finite() || epsilon < 0.0 {
            return false;
        }

        (self.x - rhs.x).abs() <= epsilon && (self.y - rhs.y).abs() <= epsilon
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
