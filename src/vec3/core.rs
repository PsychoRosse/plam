#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::unchecked_new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::unchecked_new(1.0, 1.0, 1.0);
    pub const UP: Self = Self::unchecked_new(0.0, 1.0, 0.0);
    pub const DOWN: Self = Self::unchecked_new(0.0, -1.0, 0.0);
    pub const LEFT: Self = Self::unchecked_new(-1.0, 0.0, 0.0);
    pub const RIGHT: Self = Self::unchecked_new(1.0, 0.0, 0.0);
    pub const FORWARD: Self = Self::unchecked_new(0.0, 0.0, -1.0);
    pub const BACKWARD: Self = Self::unchecked_new(0.0, 0.0, 1.0);
}

impl Vec3 {
    #[inline]
    #[must_use]
    pub fn is_approx_eq(self, rhs: Self, epsilon: f32) -> bool {
        if !self.is_finite() || !rhs.is_finite() || !epsilon.is_finite() || epsilon < 0.0 {
            return false;
        }

        (self.x - rhs.x).abs() <= epsilon
            && (self.y - rhs.y).abs() <= epsilon
            && (self.z - rhs.z).abs() <= epsilon
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
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

#[cfg(feature = "serde")]
impl Serialize for Vec3 {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(SerError::custom("Vec3 coordinates must be finite"));
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
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawVec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        let raw = RawVec3::deserialize(deserializer)?;

        Vec3::checked_new(raw.x, raw.y, raw.z).map_err(DeError::custom)
    }
}
