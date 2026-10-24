use crate::{QuatError, Vec3};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
    pub const IDENTITY: Self = Self::new_unchecked(0.0, 0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::new_unchecked(0.0, 0.0, 0.0, 0.0);
}

impl Quat {
    #[inline]
    #[must_use]
    const fn new_unchecked(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, QuatError> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || !w.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x, y, z, w))
    }
}

impl Quat {
    #[inline]
    #[must_use]
    pub const fn from_array(values: [f32; 4]) -> Result<Self, QuatError> {
        Self::new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    #[must_use]
    pub const fn splat(value: f32) -> Result<Self, QuatError> {
        Self::new(value, value, value, value)
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
    pub fn xyz(self) -> Result<Vec3, QuatError> {
        Vec3::new(self.x, self.y, self.z).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> Result<f32, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let result = self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w;

        if result.is_finite() {
            Ok(result)
        } else {
            Err(QuatError::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> Result<f32, QuatError> {
        self.dot(self)
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> Result<f32, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

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

        if result.is_finite() {
            Ok(result)
        } else {
            Err(QuatError::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Err(QuatError::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;

        let length = (x * x + y * y + z * z + w * w).sqrt();

        Self::new(x / length, y / length, z / length, w / length)
            .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn conjugate(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(-self.x, -self.y, -self.z, self.w).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn inverse(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Err(QuatError::ZeroLength);
        }

        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        let w = self.w as f64;

        let norm_squared = x * x + y * y + z * z + w * w;

        if norm_squared == 0.0 {
            return Err(QuatError::ZeroLength);
        }

        let result = [
            (-x / norm_squared) as f32,
            (-y / norm_squared) as f32,
            (-z / norm_squared) as f32,
            (w / norm_squared) as f32,
        ];

        Self::new(result[0], result[1], result[2], result[3]).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn add(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn sub(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn mul(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn scale(self, scalar: f32) -> Result<Self, QuatError> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn div_scalar(self, scalar: f32) -> Result<Self, QuatError> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(QuatError::DivisionByZero);
        }

        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn neg(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Self::new(-self.x, -self.y, -self.z, -self.w).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Result<Self, QuatError> {
        if !x_degrees.is_finite() || !y_degrees.is_finite() || !z_degrees.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let x = x_degrees.rem_euclid(360.0).to_radians();
        let y = y_degrees.rem_euclid(360.0).to_radians();
        let z = z_degrees.rem_euclid(360.0).to_radians();

        let (sin_x, cos_x) = (x * 0.5).sin_cos();
        let (sin_y, cos_y) = (y * 0.5).sin_cos();
        let (sin_z, cos_z) = (z * 0.5).sin_cos();

        Self::new(
            sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z + sin_x * sin_y * cos_z,
            cos_x * cos_y * cos_z - sin_x * sin_y * sin_z,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, degrees: f32) -> Result<Self, QuatError> {
        if !axis.is_finite() || !degrees.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());

        if scale == 0.0 {
            return Err(QuatError::InvalidAxis);
        }

        let x = axis.x / scale;
        let y = axis.y / scale;
        let z = axis.z / scale;
        let length = (x * x + y * y + z * z).sqrt();

        let half_angle = degrees.rem_euclid(360.0).to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        Self::new(
            (x / length) * sin,
            (y / length) * sin,
            (z / length) * sin,
            cos,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn from_scaled_axis(scaled_axis: Vec3) -> Result<Self, QuatError> {
        if !scaled_axis.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = scaled_axis
            .x
            .abs()
            .max(scaled_axis.y.abs())
            .max(scaled_axis.z.abs());

        if scale == 0.0 {
            return Ok(Self::IDENTITY);
        }

        let x = scaled_axis.x / scale;
        let y = scaled_axis.y / scale;
        let z = scaled_axis.z / scale;
        let scaled_length = (x * x + y * y + z * z).sqrt();

        let angle = (scale as f64) * (scaled_length as f64);
        let angle = angle.rem_euclid(360.0) as f32;

        let half_angle = angle.to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        Self::new(
            (x / scaled_length) * sin,
            (y / scaled_length) * sin,
            (z / scaled_length) * sin,
            cos,
        )
        .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_x(degrees: f32) -> Result<Self, QuatError> {
        Self::from_axis_angle(Vec3::RIGHT, degrees)
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_y(degrees: f32) -> Result<Self, QuatError> {
        Self::from_axis_angle(Vec3::UP, degrees)
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_z(degrees: f32) -> Result<Self, QuatError> {
        Self::from_axis_angle(Vec3::DOWN, degrees)
    }

    #[inline]
    #[must_use]
    pub fn rotate_vec3(self, vector: Vec3) -> Result<Vec3, QuatError> {
        if !self.is_finite() || !vector.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let q = self.normalize()?;

        let (x, y, z) = (vector.x as f64, vector.y as f64, vector.z as f64);
        let (qx, qy, qz, qw) = (q.x as f64, q.y as f64, q.z as f64, q.w as f64);

        let tx = 2.0 * (qy * z - qz * y);
        let ty = 2.0 * (qz * x - qx * z);
        let tz = 2.0 * (qx * y - qy * x);

        let rx = x + qw * tx + qy * tz - qz * ty;
        let ry = y + qw * ty + qz * tx - qx * tz;
        let rz = z + qw * tz + qx * ty - qy * tx;

        let limit = f32::MAX as f64;

        if !rx.is_finite()
            || !ry.is_finite()
            || !rz.is_finite()
            || rx.abs() > limit
            || ry.abs() > limit
            || rz.abs() > limit
        {
            return Err(QuatError::InvalidResult);
        }

        Vec3::new(rx as f32, ry as f32, rz as f32).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    #[must_use]
    pub fn to_axis_angle(self) -> Result<(Vec3, f32), QuatError> {
        let mut q = self.normalize()?;

        if q.w < 0.0 {
            q = q.neg()?;
        }

        let sin_half = q.x.hypot(q.y).hypot(q.z);

        if !sin_half.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        if sin_half == 0.0 {
            let axis = Vec3::new(1.0, 0.0, 0.0).map_err(|_| QuatError::InvalidResult)?;
            return Ok((axis, 0.0));
        }

        let axis = Vec3::new(q.x / sin_half, q.y / sin_half, q.z / sin_half)
            .map_err(|_| QuatError::InvalidResult)?;

        let angle = 2.0 * sin_half.atan2(q.w).to_degrees();

        if !angle.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok((axis, angle))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Quat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(serde::ser::Error::custom(
                "Quat contains a non-finite component",
            ));
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
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawQuat {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        let raw = RawQuat::deserialize(deserializer)?;

        Self::new(raw.x, raw.y, raw.z, raw.w).map_err(serde::de::Error::custom)
    }
}
