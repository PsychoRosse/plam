use crate::{Vec3, Vec3Error, Vec4Error};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self::unchecked_new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Self = Self::unchecked_new(1.0, 1.0, 1.0, 1.0);
    pub const UP: Self = Self::unchecked_new(0.0, 1.0, 0.0, 0.0);
    pub const DOWN: Self = Self::unchecked_new(0.0, -1.0, 0.0, 0.0);
    pub const LEFT: Self = Self::unchecked_new(-1.0, 0.0, 0.0, 0.0);
    pub const RIGHT: Self = Self::unchecked_new(1.0, 0.0, 0.0, 0.0);
    pub const FORWARD: Self = Self::unchecked_new(0.0, 0.0, -1.0, 0.0);
    pub const BACKWARD: Self = Self::unchecked_new(0.0, 0.0, 1.0, 0.0);
}

impl Vec4 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn checked_new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, Vec4Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || !w.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y, z, w))
    }

    #[inline]
    pub const fn checked_splat(value: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(value, value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value, value, value)
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
    pub const fn z(self) -> f32 {
        self.z
    }

    #[inline]
    #[must_use]
    pub const fn w(self) -> f32 {
        self.w
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub const fn checked_with_x(self, x: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(x, self.y, self.z, self.w)
    }

    #[inline]
    pub const fn checked_with_y(self, y: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(self.x, y, self.z, self.w)
    }

    #[inline]
    pub const fn checked_with_z(self, z: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(self.x, self.y, z, self.w)
    }

    #[inline]
    pub const fn checked_with_w(self, w: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(self.x, self.y, self.z, w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_with_x(self, x: f32) -> Self {
        Self::unchecked_new(x, self.y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_with_y(self, y: f32) -> Self {
        Self::unchecked_new(self.x, y, self.z, self.w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_with_z(self, z: f32) -> Self {
        Self::unchecked_new(self.x, self.y, z, self.w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_with_w(self, w: f32) -> Self {
        Self::unchecked_new(self.x, self.y, self.z, w)
    }

    #[inline]
    pub const fn checked_xyz(self) -> Result<Vec3, Vec3Error> {
        Vec3::checked_new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_xyz(self) -> Vec3 {
        Vec3::unchecked_new(self.x, self.y, self.z)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(SerError::custom("Vec4 coordinates must be finite"));
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
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawVec4 {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        let raw = RawVec4::deserialize(deserializer)?;

        Vec4::checked_new(raw.x, raw.y, raw.z, raw.w).map_err(DeError::custom)
    }
}
