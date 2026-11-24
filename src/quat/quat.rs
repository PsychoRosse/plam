#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    pub const IDENTITY: Self = Self::unchecked_new(0.0, 0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::unchecked_new(0.0, 0.0, 0.0, 0.0);
}

impl Quat {
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
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
    pub const fn is_normalized(self) -> bool {
        if !self.is_finite() {
            return false;
        }

        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        let w = self.w as f64;

        let length_squared = (x * x + y * y + z * z + w * w) as f32;

        (length_squared - 1.0).abs() <= 1e-6
    }

    #[inline]
    #[must_use]
    pub const fn is_approx_eq(self, rhs: Self, epsilon: f32) -> bool {
        if !self.is_finite() || !rhs.is_finite() || !epsilon.is_finite() || epsilon < 0.0 {
            return false;
        }

        (self.x - rhs.x).abs() <= epsilon
            && (self.y - rhs.y).abs() <= epsilon
            && (self.z - rhs.z).abs() <= epsilon
            && (self.w - rhs.w).abs() <= epsilon
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
}

#[cfg(feature = "serde")]
impl Serialize for Quat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(SerError::custom("Quat contains a non-finite component"));
        }

        let mut state = serializer.serialize_struct("Quat", 4)?;

        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.serialize_field("w", &self.w)?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Quat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawQuat {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        let raw = RawQuat::deserialize(deserializer)?;

        Quat::checked_new(raw.x, raw.y, raw.z, raw.w).map_err(DeError::custom)
    }
}
