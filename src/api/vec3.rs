use crate::Vec3Error;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[doc = include_str!("../../docs/api/Vec3.md")]
pub struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new_unchecked(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new_unchecked(1.0, 1.0, 1.0);

    pub const UP: Self = Self::new_unchecked(0.0, 1.0, 0.0);
    pub const DOWN: Self = Self::new_unchecked(0.0, -1.0, 0.0);

    pub const LEFT: Self = Self::new_unchecked(-1.0, 0.0, 0.0);
    pub const RIGHT: Self = Self::new_unchecked(1.0, 0.0, 0.0);
    pub const FORWARD: Self = Self::new_unchecked(0.0, 0.0, -1.0);
    pub const BACKWARD: Self = Self::new_unchecked(0.0, 0.0, 1.0);
}

impl Vec3 {
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.new.md")]
    pub const fn new(x: f32, y: f32, z: f32) -> Result<Self, Vec3Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y, z))
    }
}

impl Vec3 {
    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.splat.md")]
    pub const fn splat(value: f32) -> Result<Self, Vec3Error> {
        Self::new(value, value, value)
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.is_finite.md")]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.is_zero.md")]
    pub const fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.x.md")]
    pub const fn x(self) -> f32 {
        self.x
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.y.md")]
    pub const fn y(self) -> f32 {
        self.y
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.z.md")]
    pub const fn z(self) -> f32 {
        self.z
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.length_squared.md")]
    pub const fn length_squared(self) -> Result<f32, Vec3Error> {
        if !self.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let result = self.x * self.x + self.y * self.y + self.z * self.z;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.length.md")]
    pub fn length(self) -> Result<f32, Vec3Error> {
        self.ensure_finite()?;

        let scale = self.x.abs().max(self.y.abs()).max(self.z.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;

        let scaled_length = (x * x + y * y + z * z).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.dot.md")]
    pub const fn dot(self, other: Self) -> Result<f32, Vec3Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let result = self.x as f64 * other.x as f64
            + self.y as f64 * other.y as f64
            + self.z as f64 * other.z as f64;

        let result = result as f32;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.cross.md")]
    pub const fn cross(self, other: Self) -> Result<Self, Vec3Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let x = self.y as f64 * other.z as f64 - self.z as f64 * other.y as f64;
        let y = self.z as f64 * other.x as f64 - self.x as f64 * other.z as f64;
        let z = self.x as f64 * other.y as f64 - self.y as f64 * other.x as f64;

        match Self::new(x as f32, y as f32, z as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec3Error::InvalidResult),
        }
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.normalize.md")]
    pub fn normalize(self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;

        let scale = self.x.abs().max(self.y.abs()).max(self.z.abs());

        if scale == 0.0 {
            return Err(Vec3Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;

        let length = (x * x + y * y + z * z).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec3Error::InvalidResult);
        }

        let result = Self::new_unchecked(x / length, y / length, z / length);

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.distance.md")]
    pub fn distance(self, other: Self) -> Result<f32, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;

        let result = (dx * dx + dy * dy + dz * dz).sqrt() as f32;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Vec3.lerp.md")]
    pub const fn lerp(self, other: Self, t: f32) -> Result<Self, Vec3Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        if !t.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = (self.x as f64) + (other.x as f64 - self.x as f64) * t;
        let y = (self.y as f64) + (other.y as f64 - self.y as f64) * t;
        let z = (self.z as f64) + (other.z as f64 - self.z as f64) * t;

        match Self::new(x as f32, y as f32, z as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec3Error::InvalidResult),
        }
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Vec3.to_array.md")]
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    const fn ensure_finite(self) -> Result<(), Vec3Error> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(Vec3Error::NonFiniteInput)
        }
    }
}

impl Neg for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.ensure_finite()?;

        Self::new(-self.x, -self.y, -self.z).map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Add for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Sub for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Mul for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Div for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        self.ensure_finite()?;
        other.ensure_finite()?;

        if other.x == 0.0 || other.y == 0.0 || other.z == 0.0 {
            return Err(Vec3Error::DivisionByZero);
        }

        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

impl Div<f32> for Vec3 {
    type Output = Result<Self, Vec3Error>;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Vec3Error::DivisionByZero);
        }

        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
            .map_err(|_| Vec3Error::InvalidResult)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(serde::ser::Error::custom("Vec3 coordinates must be finite"));
        }

        let mut state = serializer.serialize_struct("Vec3", 3)?;

        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawVec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        let raw = RawVec3::deserialize(deserializer)?;

        Vec3::new(raw.x, raw.y, raw.z).map_err(serde::de::Error::custom)
    }
}
