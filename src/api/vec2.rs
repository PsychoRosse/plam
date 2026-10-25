use crate::Vec2Error;

use std::ops::{Add, Div, Mul, Sub};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new_unchecked(0.0, 0.0);
    pub const ONE: Self = Self::new_unchecked(1.0, 1.0);
    pub const UP: Self = Self::new_unchecked(0.0, 1.0);
    pub const DOWN: Self = Self::new_unchecked(0.0, -1.0);
    pub const LEFT: Self = Self::new_unchecked(-1.0, 0.0);
    pub const RIGHT: Self = Self::new_unchecked(1.0, 0.0);
}

impl Vec2 {
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn new(x: f32, y: f32) -> Result<Self, Vec2Error> {
        if !x.is_finite() || !y.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y))
    }
}

impl Vec2 {
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
    pub fn length_squared(self) -> Result<f32, Vec2Error> {
        self.ensure_finite()?;

        let result = self.x * self.x + self.y * self.y;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn length(self) -> Result<f32, Vec2Error> {
        self.ensure_finite()?;

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;

        let scaled_length = (x * x + y * y).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn dot(self, other: Self) -> Result<f32, Vec2Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let result = self.x * other.x + self.y * other.y;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn normalize(self) -> Result<Self, Vec2Error> {
        self.ensure_finite()?;

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return Err(Vec2Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let length = (x * x + y * y).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec2Error::InvalidResult);
        }

        let result = Self::new_unchecked(x / length, y / length);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn distance(self, other: Self) -> Result<f32, Vec2Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let delta =
            Self::new(self.x - other.x, self.y - other.y).map_err(|_| Vec2Error::InvalidResult)?;

        delta.length()
    }

    #[inline]
    pub fn perpendicular(self) -> Result<Self, Vec2Error> {
        self.ensure_finite()?;
        Ok(Self::new_unchecked(-self.y, self.x))
    }

    #[inline]
    const fn ensure_finite(self) -> Result<(), Vec2Error> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(Vec2Error::NonFiniteInput)
        }
    }
}

impl Add for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x + other.x, self.y + other.y).map_err(|_| Vec2Error::InvalidResult)
    }
}

impl Sub for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x - other.x, self.y - other.y).map_err(|_| Vec2Error::InvalidResult)
    }
}

impl Mul for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x * other.x, self.y * other.y).map_err(|_| Vec2Error::InvalidResult)
    }
}

impl Div for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        if other.x == 0.0 || other.y == 0.0 {
            return Err(Vec2Error::DivisionByZero);
        }

        Self::new(self.x / other.x, self.y / other.y).map_err(|_| Vec2Error::InvalidResult)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Self::new(self.x * scalar, self.y * scalar).map_err(|_| Vec2Error::InvalidResult)
    }
}

impl Div<f32> for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Vec2Error::DivisionByZero);
        }

        Self::new(self.x / scalar, self.y / scalar).map_err(|_| Vec2Error::InvalidResult)
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

        Vec2::new(raw.x, raw.y).map_err(DeError::custom)
    }
}
