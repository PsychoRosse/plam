use crate::Vec2Error;

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::unchecked_new(0.0, 0.0);
    pub const ONE: Self = Self::unchecked_new(1.0, 1.0);
    pub const UP: Self = Self::unchecked_new(0.0, 1.0);
    pub const DOWN: Self = Self::unchecked_new(0.0, -1.0);
    pub const LEFT: Self = Self::unchecked_new(-1.0, 0.0);
    pub const RIGHT: Self = Self::unchecked_new(1.0, 0.0);
}

impl Vec2 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn checked_new(x: f32, y: f32) -> Result<Self, Vec2Error> {
        if !x.is_finite() || !y.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y))
    }

    #[inline]
    #[must_use]
    pub const fn checked_splat(value: f32) -> Result<Self, Vec2Error> {
        Self::checked_new(value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value)
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
    #[must_use]
    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(SerError::custom("Vec2 coordinates must be finite"));
        }

        let mut state = serializer.serialize_struct("Vec2", 2)?;

        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawVec2 {
            x: f32,
            y: f32,
        }

        let raw = RawVec2::deserialize(deserializer)?;

        Vec2::checked_new(raw.x, raw.y).map_err(DeError::custom)
    }
}
