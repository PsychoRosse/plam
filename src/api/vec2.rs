use crate::Vec2Error;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[doc = include_str!("../../docs/api/Vec2.md")]
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
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.new.md")]
    pub const fn new(x: f32, y: f32) -> Result<Self, Vec2Error> {
        if !x.is_finite() || !y.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y))
    }
}

impl Vec2 {
    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.splat.md")]
    pub const fn splat(value: f32) -> Result<Self, Vec2Error> {
        Self::new(value, value)
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec2.is_finite.md")]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec2.is_zero.md")]
    pub const fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec2.x.md")]
    pub const fn x(self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec2.y.md")]
    pub const fn y(self) -> f32 {
        self.y
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec2.to_array.md")]
    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.length_squared.md")]
    pub const fn length_squared(self) -> Result<f32, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.x * self.x + self.y * self.y;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.length.md")]
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
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.dot.md")]
    pub const fn dot(self, other: Self) -> Result<f32, Vec2Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.x as f64 * other.x as f64 + self.y as f64 * other.y as f64;

        let result = result as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.normalize.md")]
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
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.distance.md")]
    pub fn distance(self, other: Self) -> Result<f32, Vec2Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;

        let result = (dx * dx + dy * dy).sqrt() as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.lerp.md")]
    pub const fn lerp(self, other: Self, t: f32) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        if !t.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = (self.x as f64) + (other.x as f64 - self.x as f64) * t;
        let y = (self.y as f64) + (other.y as f64 - self.y as f64) * t;

        match Self::new(x as f32, y as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec2Error::InvalidResult),
        }
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec2.perpendicular.md")]
    pub const fn perpendicular(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

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

impl Neg for Vec2 {
    type Output = Result<Self, Vec2Error>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.ensure_finite()?;

        Self::new(-self.x, -self.y).map_err(|_| Vec2Error::InvalidResult)
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
