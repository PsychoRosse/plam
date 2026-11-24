use crate::{QuatError, Vec3};

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
    pub const fn unchecked_new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn checked_new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, QuatError> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || !w.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y, z, w))
    }

    #[inline]
    pub const fn checked_from_array(values: [f32; 4]) -> Result<Self, QuatError> {
        Self::checked_new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(values: [f32; 4]) -> Self {
        Self::unchecked_new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    pub const fn checked_splat(value: f32) -> Result<Self, QuatError> {
        Self::checked_new(value, value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value, value, value)
    }

    #[inline]
    pub fn checked_from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Result<Self, QuatError> {
        if !x_degrees.is_finite() || !y_degrees.is_finite() || !z_degrees.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(Self::unchecked_from_euler(x_degrees, y_degrees, z_degrees))
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
        let x = x_degrees.rem_euclid(360.0).to_radians();
        let y = y_degrees.rem_euclid(360.0).to_radians();
        let z = z_degrees.rem_euclid(360.0).to_radians();

        let (sin_x, cos_x) = (x * 0.5).sin_cos();
        let (sin_y, cos_y) = (y * 0.5).sin_cos();
        let (sin_z, cos_z) = (z * 0.5).sin_cos();

        Self::unchecked_new(
            sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z + sin_x * sin_y * cos_z,
            cos_x * cos_y * cos_z - sin_x * sin_y * sin_z,
        )
    }

    #[inline]
    pub fn checked_from_axis_angle(axis: Vec3, degrees: f32) -> Result<Self, QuatError> {
        if !axis.is_finite() || !degrees.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());

        if scale == 0.0 {
            return Err(QuatError::InvalidAxis);
        }

        Ok(Self::unchecked_from_axis_angle(axis, degrees))
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        let scale = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());

        let x = axis.x / scale;
        let y = axis.y / scale;
        let z = axis.z / scale;
        let length = (x * x + y * y + z * z).sqrt();

        let half_angle = degrees.rem_euclid(360.0).to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        Self::unchecked_new((x / length) * sin, (y / length) * sin, (z / length) * sin, cos)
    }

    #[inline]
    pub fn checked_from_scaled_axis(scaled_axis: Vec3) -> Result<Self, QuatError> {
        if !scaled_axis.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(Self::unchecked_from_scaled_axis(scaled_axis))
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_scaled_axis(scaled_axis: Vec3) -> Self {
        let scale = scaled_axis
            .x
            .abs()
            .max(scaled_axis.y.abs())
            .max(scaled_axis.z.abs());

        if scale == 0.0 {
            return Self::IDENTITY;
        }

        let x = scaled_axis.x / scale;
        let y = scaled_axis.y / scale;
        let z = scaled_axis.z / scale;

        let scaled_length = (x * x + y * y + z * z).sqrt();

        let angle = (scale as f64) * (scaled_length as f64);
        let half_angle = angle.rem_euclid(360.0).to_radians() * 0.5;

        let (sin, cos) = half_angle.sin_cos();

        let sin = sin as f32;
        let cos = cos as f32;

        Self::unchecked_new(
            (x / scaled_length) * sin,
            (y / scaled_length) * sin,
            (z / scaled_length) * sin,
            cos,
        )
    }

    #[inline]
    pub fn checked_from_rotation_x(degrees: f32) -> Result<Self, QuatError> {
        Self::checked_from_axis_angle(Vec3::RIGHT, degrees)
    }

    #[inline]
    pub fn checked_from_rotation_y(degrees: f32) -> Result<Self, QuatError> {
        Self::checked_from_axis_angle(Vec3::UP, degrees)
    }

    #[inline]
    pub fn checked_from_rotation_z(degrees: f32) -> Result<Self, QuatError> {
        Self::checked_from_axis_angle(Vec3::BACKWARD, degrees)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_rotation_x(degrees: f32) -> Self {
        Self::unchecked_from_axis_angle(Vec3::RIGHT, degrees)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_rotation_y(degrees: f32) -> Self {
        Self::unchecked_from_axis_angle(Vec3::UP, degrees)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_rotation_z(degrees: f32) -> Self {
        Self::unchecked_from_axis_angle(Vec3::BACKWARD, degrees)
    }

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

    #[inline]
    pub const fn checked_xyz(self) -> Result<Vec3, QuatError> {
        match Vec3::checked_new(self.x, self.y, self.z) {
            Ok(value) => Ok(value),
            Err(_) => Err(QuatError::InvalidResult),
        }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_xyz(self) -> Vec3 {
        Vec3::unchecked_new(self.x, self.y, self.z)
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
