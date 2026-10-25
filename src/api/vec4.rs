use crate::{Vec3, Vec3Error, Vec4Error};

use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self::new_unchecked(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new_unchecked(1.0, 1.0, 1.0, 1.0);

    pub const UP: Self = Self::new_unchecked(0.0, 1.0, 0.0, 0.0);
    pub const DOWN: Self = Self::new_unchecked(0.0, -1.0, 0.0, 0.0);
    pub const LEFT: Self = Self::new_unchecked(-1.0, 0.0, 0.0, 0.0);
    pub const RIGHT: Self = Self::new_unchecked(1.0, 0.0, 0.0, 0.0);

    pub const FORWARD: Self = Self::new_unchecked(0.0, 0.0, -1.0, 0.0);
    pub const BACKWARD: Self = Self::new_unchecked(0.0, 0.0, 1.0, 0.0);
}

impl Vec4 {
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, Vec4Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || !w.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y, z, w))
    }
}

impl Vec4 {
    #[inline]
    pub fn splat(value: f32) -> Result<Self, Vec4Error> {
        Self::new(value, value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub fn length_squared(self) -> Result<f32, Vec4Error> {
        self.ensure_finite()?;

        let result = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn length(self) -> Result<f32, Vec4Error> {
        self.ensure_finite()?;

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;

        let scaled_length = (x * x + y * y + z * z + w * w).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn dot(self, other: Self) -> Result<f32, Vec4Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let result = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn normalize(self) -> Result<Self, Vec4Error> {
        self.ensure_finite()?;

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Err(Vec4Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;

        let length = (x * x + y * y + z * z + w * w).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec4Error::InvalidResult);
        }

        let result = Self::new_unchecked(x / length, y / length, z / length, w / length);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn abs(self) -> Result<Self, Vec4Error> {
        self.ensure_finite()?;

        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
            .map_err(|_| Vec4Error::InvalidResult)
    }

    #[inline]
    pub fn min(self, other: Self) -> Result<Self, Vec4Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }

    #[inline]
    pub fn max(self, other: Self) -> Result<Self, Vec4Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }

    #[inline]
    pub fn xyz(self) -> Result<Vec3, Vec3Error> {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn distance(self, other: Self) -> Result<f32, Vec4Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let delta = Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
        .map_err(|_| Vec4Error::InvalidResult)?;

        delta.length()
    }

    #[inline]
    pub(crate) fn ensure_finite(self) -> Result<(), Vec4Error> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(Vec4Error::NonFiniteInput)
        }
    }
}

impl Neg for Vec4 {
    type Output = Result<Self, Vec4Error>;
    #[inline]
    fn neg(self) -> Self::Output {
        self.ensure_finite()?;

        Self::new(-self.x, -self.y, -self.z, -self.w).map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Add for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Sub for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Mul for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Div for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        if other.x == 0.0 || other.y == 0.0 || other.z == 0.0 || other.w == 0.0 {
            return Err(Vec4Error::DivisionByZero);
        }

        Self::new(
            self.x / other.x,
            self.y / other.y,
            self.z / other.z,
            self.w / other.w,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

impl Div<f32> for Vec4 {
    type Output = Result<Self, Vec4Error>;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Vec4Error::DivisionByZero);
        }

        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
        .map_err(|_| Vec4Error::InvalidResult)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(serde::ser::Error::custom("Vec4 coordinates must be finite"));
        }

        let mut state = serializer.serialize_struct("Vec4", 4)?;

        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawVec4 {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        let raw = RawVec4::deserialize(deserializer)?;

        Vec4::new(raw.x, raw.y, raw.z, raw.w).map_err(serde::de::Error::custom)
    }
}
