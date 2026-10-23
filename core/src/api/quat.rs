use std::ops::{Div, DivAssign, Mul, MulAssign, Neg};

use crate::Vec3;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quat {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

impl Quat {
    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Quat {
    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    #[must_use]
    pub const fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn length_stable(self) -> f32 {
        if !self.is_finite() {
            return f32::NAN;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return 0.0;
        }

        let scaled = self / scale;
        scale * scaled.length()
    }

    #[inline]
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return None;
        }

        let scaled = Self::new(
            self.x / scale,
            self.y / scale,
            self.z / scale,
            self.w / scale,
        );

        Some(scaled / scaled.length())
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    #[must_use]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        self.try_inverse()
            .expect("cannot invert a zero or invalid quaternion")
    }

    #[inline]
    pub fn try_inverse(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return None;
        }

        let scaled = Self::new(
            self.x / scale,
            self.y / scale,
            self.z / scale,
            self.w / scale,
        );

        let norm_squared = scaled.length_squared();

        let inverse = scaled.conjugate() / norm_squared / scale;

        inverse.is_finite().then_some(inverse)
    }

    #[inline]
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        let q = self
            .try_normalize()
            .expect("cannot convert an invalid quaternion to axis-angle");

        let q = if q.w < 0.0 { -q } else { q };

        let sin_half = q.x.hypot(q.y).hypot(q.z);

        if sin_half == 0.0 {
            return (Vec3::RIGHT, 0.0);
        }

        let axis = Vec3::new(q.x, q.y, q.z) / sin_half;
        let angle = 2.0 * sin_half.atan2(q.w).to_degrees();

        (axis, angle)
    }

    #[inline]
    pub fn try_rotate_vec3(self, vector: Vec3) -> Option<Vec3> {
        if !vector.is_finite() {
            return None;
        }

        let q = self.try_normalize()?;

        let qv = Vec3::new(q.x, q.y, q.z);
        let t = qv.cross(vector) * 2.0;
        let result = vector + t * q.w + qv.cross(t);

        result.is_finite().then_some(result)
    }

    #[inline]
    pub fn rotate_vec3(self, vector: Vec3) -> Vec3 {
        self.try_rotate_vec3(vector)
            .expect("cannot rotate a vector with an invalid quaternion or vector")
    }
}

impl Quat {
    #[inline]
    #[must_use]
    pub fn from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
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
    }

    #[inline]
    pub fn from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        Self::try_from_axis_angle(axis, degrees)
            .expect("axis-angle requires a finite non-zero axis and finite angle")
    }

    #[inline]
    pub fn try_from_axis_angle(axis: Vec3, degrees: f32) -> Option<Self> {
        if !degrees.is_finite() {
            return None;
        }

        let axis = axis.try_normalize()?;
        let reduced_degrees = degrees.rem_euclid(360.0);
        let half_angle = reduced_degrees.to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        let result = Self::new(axis.x * sin, axis.y * sin, axis.z * sin, cos);

        result.is_finite().then_some(result)
    }

    #[inline]
    pub fn from_scaled_axis(scaled_axis: Vec3) -> Self {
        Self::try_from_scaled_axis(scaled_axis).unwrap_or(Self::IDENTITY)
    }

    #[inline]
    pub fn try_from_scaled_axis(scaled_axis: Vec3) -> Option<Self> {
        if !scaled_axis.is_finite() {
            return None;
        }

        let degrees = scaled_axis.length_stable();

        if !degrees.is_finite() {
            return None;
        }

        if degrees <= f32::EPSILON {
            return Some(Self::IDENTITY);
        }

        let axis = scaled_axis.try_normalize()?;
        Self::try_from_axis_angle(axis, degrees)
    }

    #[inline]
    #[must_use]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(x, y, z, w)
    }

    #[inline]
    #[must_use]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_x(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(sin, 0.0, 0.0, cos)
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_y(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, sin, 0.0, cos)
    }

    #[inline]
    #[must_use]
    pub fn from_rotation_z(degrees: f32) -> Self {
        let radians = degrees.rem_euclid(360.0).to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, 0.0, sin, cos)
    }
}

impl Mul for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl MulAssign for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f32> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}
impl DivAssign<f32> for Quat {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Neg for Quat {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.rotate_vec3(rhs)
    }
}
