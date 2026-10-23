use crate::{Mat4Error, Quat, Vec3, Vec4};

use std::ops::{Mul, MulAssign};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mat4 {
    pub(crate) x_axis: Vec4,
    pub(crate) y_axis: Vec4,
    pub(crate) z_axis: Vec4,
    pub(crate) w_axis: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Self = Self::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
}

impl Mat4 {
    #[inline]
    #[must_use]
    pub const fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }
}

impl Mat4 {
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self::new(x_axis, y_axis, z_axis, w_axis)
    }

    #[inline]
    #[must_use]
    pub const fn from_array(array: [f32; 16]) -> Self {
        Self::new(
            Vec4::new(array[0], array[1], array[2], array[3]),
            Vec4::new(array[4], array[5], array[6], array[7]),
            Vec4::new(array[8], array[9], array[10], array[11]),
            Vec4::new(array[12], array[13], array[14], array[15]),
        )
    }

    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(
            Vec4::new(scale.x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.z, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Result<Self, Mat4Error> {
        if !rotation.x.is_finite()
            || !rotation.y.is_finite()
            || !rotation.z.is_finite()
            || !rotation.w.is_finite()
        {
            return Err(Mat4Error::NonFiniteInput);
        }

        let length_squared = rotation.x * rotation.x
            + rotation.y * rotation.y
            + rotation.z * rotation.z
            + rotation.w * rotation.w;

        if !length_squared.is_finite() || length_squared <= f32::MIN_POSITIVE {
            return Err(Mat4Error::InvalidQuaternion);
        }

        let q = rotation.normalize();
        if !q.x.is_finite() || !q.y.is_finite() || !q.z.is_finite() || !q.w.is_finite() {
            return Err(Mat4Error::InvalidQuaternion);
        }

        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;
        let xx = q.x * x2;
        let xy = q.x * y2;
        let xz = q.x * z2;
        let yy = q.y * y2;
        let yz = q.y * z2;
        let zz = q.z * z2;
        let wx = q.w * x2;
        let wy = q.w * y2;
        let wz = q.w * z2;

        let matrix = Self::new(
            Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
            Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
            Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        if matrix.is_finite() {
            Ok(matrix)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub fn from_scale_rotation_translation(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Result<Self, Mat4Error> {
        if !scale.is_finite() || !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let rotation = Self::from_quat(rotation)?;
        let matrix = Self::new(
            rotation.x_axis * scale.x,
            rotation.y_axis * scale.y,
            rotation.z_axis * scale.z,
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        );

        if matrix.is_finite() {
            Ok(matrix)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f32; 16] {
        [
            self.x_axis.x,
            self.x_axis.y,
            self.x_axis.z,
            self.x_axis.w,
            self.y_axis.x,
            self.y_axis.y,
            self.y_axis.z,
            self.y_axis.w,
            self.z_axis.x,
            self.z_axis.y,
            self.z_axis.z,
            self.z_axis.w,
            self.w_axis.x,
            self.w_axis.y,
            self.w_axis.z,
            self.w_axis.w,
        ]
    }

    #[inline]
    #[must_use]
    pub const fn to_cols_array_2d(self) -> [[f32; 4]; 4] {
        [
            [self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w],
            [self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w],
            [self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w],
            [self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w],
        ]
    }

    #[inline]
    #[must_use]
    pub fn transform_point3(self, point: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !point.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self * Vec4::new(point.x, point.y, point.z, 1.0);
        if !result.is_finite() || result.w == 0.0 {
            return Err(Mat4Error::InvalidResult);
        }

        let point = Vec3::new(
            result.x / result.w,
            result.y / result.w,
            result.z / result.w,
        );

        if point.is_finite() {
            Ok(point)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub fn transform_vector3(self, vector: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !vector.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self * Vec4::new(vector.x, vector.y, vector.z, 0.0);
        let vector = Vec3::new(result.x, result.y, result.z);

        if vector.is_finite() {
            Ok(vector)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub const fn transpose(self) -> Self {
        Self::new(
            Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        )
    }

    #[inline]
    #[must_use]
    pub fn determinant(self) -> f32 {
        let m = self.to_array();

        let a0 = m[0] * m[5] - m[1] * m[4];
        let a1 = m[0] * m[6] - m[2] * m[4];
        let a2 = m[0] * m[7] - m[3] * m[4];
        let a3 = m[1] * m[6] - m[2] * m[5];
        let a4 = m[1] * m[7] - m[3] * m[5];
        let a5 = m[2] * m[7] - m[3] * m[6];

        let b0 = m[8] * m[13] - m[9] * m[12];
        let b1 = m[8] * m[14] - m[10] * m[12];
        let b2 = m[8] * m[15] - m[11] * m[12];
        let b3 = m[9] * m[14] - m[10] * m[13];
        let b4 = m[9] * m[15] - m[11] * m[13];
        let b5 = m[10] * m[15] - m[11] * m[14];

        a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0
    }

    #[inline]
    #[must_use]
    pub fn inverse(self) -> Result<Self, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let m = self.to_array();

        let a0 = m[0] * m[5] - m[1] * m[4];
        let a1 = m[0] * m[6] - m[2] * m[4];
        let a2 = m[0] * m[7] - m[3] * m[4];
        let a3 = m[1] * m[6] - m[2] * m[5];
        let a4 = m[1] * m[7] - m[3] * m[5];
        let a5 = m[2] * m[7] - m[3] * m[6];

        let b0 = m[8] * m[13] - m[9] * m[12];
        let b1 = m[8] * m[14] - m[10] * m[12];
        let b2 = m[8] * m[15] - m[11] * m[12];
        let b3 = m[9] * m[14] - m[10] * m[13];
        let b4 = m[9] * m[15] - m[11] * m[13];
        let b5 = m[10] * m[15] - m[11] * m[14];

        let determinant = a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0;

        if !determinant.is_finite() || determinant == 0.0 {
            return Err(Mat4Error::SingularMatrix);
        }

        let inv_det = 1.0 / determinant;
        if !inv_det.is_finite() {
            return Err(Mat4Error::InvalidResult);
        }

        let inverse = Self::new(
            Vec4::new(
                (m[5] * b5 - m[6] * b4 + m[7] * b3) * inv_det,
                (-m[1] * b5 + m[2] * b4 - m[3] * b3) * inv_det,
                (m[13] * a5 - m[14] * a4 + m[15] * a3) * inv_det,
                (-m[9] * a5 + m[10] * a4 - m[11] * a3) * inv_det,
            ),
            Vec4::new(
                (-m[4] * b5 + m[6] * b2 - m[7] * b1) * inv_det,
                (m[0] * b5 - m[2] * b2 + m[3] * b1) * inv_det,
                (-m[12] * a5 + m[14] * a2 - m[15] * a1) * inv_det,
                (m[8] * a5 - m[10] * a2 + m[11] * a1) * inv_det,
            ),
            Vec4::new(
                (m[4] * b4 - m[5] * b2 + m[7] * b0) * inv_det,
                (-m[0] * b4 + m[1] * b2 - m[3] * b0) * inv_det,
                (m[12] * a4 - m[13] * a2 + m[15] * a0) * inv_det,
                (-m[8] * a4 + m[9] * a2 - m[11] * a0) * inv_det,
            ),
            Vec4::new(
                (-m[4] * b3 + m[5] * b1 - m[6] * b0) * inv_det,
                (m[0] * b3 - m[1] * b1 + m[2] * b0) * inv_det,
                (-m[12] * a3 + m[13] * a1 - m[14] * a0) * inv_det,
                (m[8] * a3 - m[9] * a1 + m[10] * a0) * inv_det,
            ),
        );

        if inverse.is_finite() {
            Ok(inverse)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }

    #[inline]
    #[must_use]
    pub fn perspective_rh(
        fov_y_degrees: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Result<Self, Mat4Error> {
        if !fov_y_degrees.is_finite()
            || !aspect_ratio.is_finite()
            || !near.is_finite()
            || !far.is_finite()
            || !(0.0 < fov_y_degrees && fov_y_degrees < 180.0)
            || aspect_ratio <= 0.0
            || near <= 0.0
            || far <= near
        {
            return Err(Mat4Error::InvalidPerspectiveParameters);
        }

        let f = 1.0 / (fov_y_degrees.to_radians() * 0.5).tan();
        let matrix = Self::new(
            Vec4::new(f / aspect_ratio, 0.0, 0.0, 0.0),
            Vec4::new(0.0, f, 0.0, 0.0),
            Vec4::new(0.0, 0.0, far / (near - far), -1.0),
            Vec4::new(0.0, 0.0, (near * far) / (near - far), 0.0),
        );

        if matrix.is_finite() {
            Ok(matrix)
        } else {
            Err(Mat4Error::InvalidResult)
        }
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self * rhs.x_axis,
            self * rhs.y_axis,
            self * rhs.z_axis,
            self * rhs.w_axis,
        )
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.x_axis.x * rhs.x
                + self.y_axis.x * rhs.y
                + self.z_axis.x * rhs.z
                + self.w_axis.x * rhs.w,
            self.x_axis.y * rhs.x
                + self.y_axis.y * rhs.y
                + self.z_axis.y * rhs.z
                + self.w_axis.y * rhs.w,
            self.x_axis.z * rhs.x
                + self.y_axis.z * rhs.y
                + self.z_axis.z * rhs.z
                + self.w_axis.z * rhs.w,
            self.x_axis.w * rhs.x
                + self.y_axis.w * rhs.y
                + self.z_axis.w * rhs.z
                + self.w_axis.w * rhs.w,
        )
    }
}

impl MulAssign for Mat4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
