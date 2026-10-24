#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

use crate::Vec3Error;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    const fn new_unchecked(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Result<Self, Vec3Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y, z))
    }
}

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn splat(value: f32) -> Result<Self, Vec3Error> {
        Self::new(value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> Result<f32, Vec3Error> {
        self.ensure_finite()?;

        let result = self.x * self.x + self.y * self.y + self.z * self.z;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub fn try_length(self) -> Result<f32, Vec3Error> {
        self.length_stable()
    }

    #[inline]
    #[must_use]
    pub fn length_stable(self) -> Result<f32, Vec3Error> {
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
    #[must_use]
    pub fn dot(self, other: Self) -> Result<f32, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let result = self.x * other.x + self.y * other.y + self.z * other.z;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub fn cross(self, other: Self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Self::new(x, y, z).map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Result<Self, Vec3Error> {
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
    #[must_use]
    pub fn add(self, other: Self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn sub(self, other: Self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn mul(self, other: Self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn div(self, other: Self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        if other.x == 0.0 || other.y == 0.0 || other.z == 0.0 {
            return Err(Vec3Error::DivisionByZero);
        }

        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
            .map_err(|_| Vec3Error::InvalidResult)
    }

    /// Умножает вектор на скаляр.
    #[inline]
    #[must_use]
    pub fn scale(self, scalar: f32) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;

        if !scalar.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
            .map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn div_scalar(self, scalar: f32) -> Result<Self, Vec3Error> {
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

    #[inline]
    #[must_use]
    pub fn distance(self, other: Self) -> Result<f32, Vec3Error> {
        self.ensure_finite()?;
        other.ensure_finite()?;

        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Self::new_unchecked(x, y, z).try_length()
    }

    #[inline]
    #[must_use]
    pub fn neg(self) -> Result<Self, Vec3Error> {
        self.ensure_finite()?;

        Self::new(-self.x, -self.y, -self.z).map_err(|_| Vec3Error::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    #[must_use]
    fn ensure_finite(self) -> Result<(), Vec3Error> {
        if self.is_finite() {
            Ok(())
        } else {
            Err(Vec3Error::NonFiniteInput)
        }
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
