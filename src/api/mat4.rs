use crate::{Mat4Error, Quat, Vec3, Vec4};

use std::ops::Mul;

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    ser::{Error, SerializeStruct},
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[doc = include_str!("../../docs/api/Mat4.md")]
pub struct Mat4 {
    pub(crate) x_axis: Vec4,
    pub(crate) y_axis: Vec4,
    pub(crate) z_axis: Vec4,
    pub(crate) w_axis: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Self = Self::new_unchecked(
        Vec4::new_unchecked(1.0, 0.0, 0.0, 0.0),
        Vec4::new_unchecked(0.0, 1.0, 0.0, 0.0),
        Vec4::new_unchecked(0.0, 0.0, 1.0, 0.0),
        Vec4::new_unchecked(0.0, 0.0, 0.0, 1.0),
    );
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mat4 {
    #[inline]
    #[must_use]
    pub(crate) const fn new_unchecked(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.new.md")]
    pub const fn new(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Result<Self, Mat4Error> {
        if !x_axis.is_finite() || !y_axis.is_finite() || !z_axis.is_finite() || !w_axis.is_finite()
        {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::new_unchecked(x_axis, y_axis, z_axis, w_axis))
    }
}

impl Mat4 {
    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Mat4.is_finite.md")]
    pub const fn is_finite(self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.from_cols.md")]
    pub const fn from_cols(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Result<Self, Mat4Error> {
        Self::new(x_axis, y_axis, z_axis, w_axis)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.from_array.md")]
    pub const fn from_array(array: [f32; 16]) -> Result<Self, Mat4Error> {
        let mut index = 0;

        while index < 16 {
            if !array[index].is_finite() {
                return Err(Mat4Error::NonFiniteInput);
            }

            index += 1;
        }

        Self::new(
            Vec4::new_unchecked(array[0], array[1], array[2], array[3]),
            Vec4::new_unchecked(array[4], array[5], array[6], array[7]),
            Vec4::new_unchecked(array[8], array[9], array[10], array[11]),
            Vec4::new_unchecked(array[12], array[13], array[14], array[15]),
        )
    }

    #[inline]
    #[must_use]
    #[doc = include_str!("../../docs/api/methods/Mat4.to_array.md")]
    pub const fn to_array(self) -> [f32; 16] {
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
    #[doc = include_str!("../../docs/api/methods/Mat4.to_cols_array_2d.md")]
    pub const fn to_cols_array_2d(self) -> [[f32; 4]; 4] {
        [
            [self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w],
            [self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w],
            [self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w],
            [self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w],
        ]
    }

    #[inline]
    const fn from_f64_array(array: [f64; 16]) -> Result<Self, Mat4Error> {
        let mut values = [0.0_f32; 16];
        let mut index = 0;

        while index < 16 {
            let value = array[index];

            if !value.is_finite() || value.abs() > f32::MAX as f64 {
                return Err(Mat4Error::InvalidResult);
            }

            values[index] = value as f32;

            if !values[index].is_finite() {
                return Err(Mat4Error::InvalidResult);
            }

            index += 1;
        }

        match Self::from_array(values) {
            Ok(matrix) => Ok(matrix),
            Err(_) => Err(Mat4Error::InvalidResult),
        }
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.from_translation.md")]
    pub const fn from_translation(translation: Vec3) -> Result<Self, Mat4Error> {
        if !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Self::new(
            Vec4::new_unchecked(1.0, 0.0, 0.0, 0.0),
            Vec4::new_unchecked(0.0, 1.0, 0.0, 0.0),
            Vec4::new_unchecked(0.0, 0.0, 1.0, 0.0),
            Vec4::new_unchecked(translation.x, translation.y, translation.z, 1.0),
        )
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.try_from_scale.md")]
    pub const fn try_from_scale(scale: Vec3) -> Result<Self, Mat4Error> {
        if !scale.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Self::new(
            Vec4::new_unchecked(scale.x, 0.0, 0.0, 0.0),
            Vec4::new_unchecked(0.0, scale.y, 0.0, 0.0),
            Vec4::new_unchecked(0.0, 0.0, scale.z, 0.0),
            Vec4::new_unchecked(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.from_quat.md")]
    pub fn from_quat(rotation: Quat) -> Result<Self, Mat4Error> {
        let q = rotation
            .normalize()
            .map_err(|_| Mat4Error::InvalidQuaternion)?;

        let [x, y, z, w] = q.to_array();

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;

        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;

        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let x_axis = Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let y_axis = Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let z_axis = Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let w_axis = Vec4::new(0.0, 0.0, 0.0, 1.0).map_err(|_| Mat4Error::InvalidResult)?;

        Self::new(x_axis, y_axis, z_axis, w_axis).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.from_scale_rotation_translation.md")]
    pub fn from_scale_rotation_translation(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Result<Self, Mat4Error> {
        if !scale.is_finite() || !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let rotation = Self::from_quat(rotation)?;

        let x_axis = Vec4::new(
            rotation.x_axis.x * scale.x,
            rotation.x_axis.y * scale.x,
            rotation.x_axis.z * scale.x,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let y_axis = Vec4::new(
            rotation.y_axis.x * scale.y,
            rotation.y_axis.y * scale.y,
            rotation.y_axis.z * scale.y,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let z_axis = Vec4::new(
            rotation.z_axis.x * scale.z,
            rotation.z_axis.y * scale.z,
            rotation.z_axis.z * scale.z,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let w_axis = Vec4::new(translation.x, translation.y, translation.z, 1.0)
            .map_err(|_| Mat4Error::InvalidResult)?;

        Self::new(x_axis, y_axis, z_axis, w_axis).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.transform_point3.md")]
    pub fn transform_point3(self, point: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !point.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.mul(
            Vec4::new(point.x, point.y, point.z, 1.0).map_err(|_| Mat4Error::InvalidResult)?,
        )?;

        if result.w == 0.0 {
            return Err(Mat4Error::ZeroHomogeneousCoordinate);
        }

        let x = result.x as f64 / result.w as f64;
        let y = result.y as f64 / result.w as f64;
        let z = result.z as f64 / result.w as f64;

        if !x.is_finite()
            || !y.is_finite()
            || !z.is_finite()
            || x.abs() > f32::MAX as f64
            || y.abs() > f32::MAX as f64
            || z.abs() > f32::MAX as f64
        {
            return Err(Mat4Error::InvalidResult);
        }

        Vec3::new(x as f32, y as f32, z as f32).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.transform_vector3.md")]
    pub fn transform_vector3(self, vector: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !vector.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.mul(
            Vec4::new(vector.x, vector.y, vector.z, 0.0).map_err(|_| Mat4Error::InvalidResult)?,
        )?;

        Vec3::new(result.x, result.y, result.z).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.transpose.md")]
    pub const fn transpose(self) -> Result<Self, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let a = self.to_cols_array_2d();

        Self::new(
            Vec4::new_unchecked(a[0][0], a[1][0], a[2][0], a[3][0]),
            Vec4::new_unchecked(a[0][1], a[1][1], a[2][1], a[3][1]),
            Vec4::new_unchecked(a[0][2], a[1][2], a[2][2], a[3][2]),
            Vec4::new_unchecked(a[0][3], a[1][3], a[2][3], a[3][3]),
        )
    }

    #[inline]
    #[allow(clippy::needless_range_loop)]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.determinant.md")]
    pub const fn determinant(self) -> Result<f32, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let array = self.to_array();
        let mut a = [[0.0_f64; 4]; 4];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                a[row][col] = array[col * 4 + row] as f64;
                col += 1;
            }

            row += 1;
        }

        let mut determinant = 1.0_f64;
        let mut col = 0;

        while col < 4 {
            let mut pivot = col;
            let mut scan = col + 1;

            while scan < 4 {
                if a[scan][col].abs() > a[pivot][col].abs() {
                    pivot = scan;
                }

                scan += 1;
            }

            if a[pivot][col] == 0.0 {
                return Ok(0.0);
            }

            if pivot != col {
                a.swap(pivot, col);
                determinant = -determinant;
            }

            let pivot_value = a[col][col];
            determinant *= pivot_value;

            let mut target = col + 1;

            while target < 4 {
                let factor = a[target][col] / pivot_value;
                let mut k = col + 1;

                while k < 4 {
                    a[target][k] -= factor * a[col][k];
                    k += 1;
                }

                target += 1;
            }

            col += 1;
        }

        if !determinant.is_finite() || determinant.abs() > f32::MAX as f64 {
            return Err(Mat4Error::InvalidResult);
        }

        Ok(determinant as f32)
    }

    #[inline]
    #[allow(clippy::needless_range_loop)]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.inverse.md")]
    pub const fn inverse(self) -> Result<Self, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let array = self.to_array();
        let mut a = [[0.0_f64; 8]; 4];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                a[row][col] = array[col * 4 + row] as f64;
                col += 1;
            }

            a[row][row + 4] = 1.0;
            row += 1;
        }

        let mut col = 0;

        while col < 4 {
            let mut pivot = col;
            let mut scan = col + 1;

            while scan < 4 {
                if a[scan][col].abs() > a[pivot][col].abs() {
                    pivot = scan;
                }

                scan += 1;
            }

            if a[pivot][col] == 0.0 {
                return Err(Mat4Error::SingularMatrix);
            }

            a.swap(pivot, col);

            let divisor = a[col][col];
            let mut k = 0;

            while k < 8 {
                a[col][k] /= divisor;
                k += 1;
            }

            let mut elim = 0;

            while elim < 4 {
                if elim != col {
                    let factor = a[elim][col];
                    let mut idx = 0;

                    while idx < 8 {
                        a[elim][idx] -= factor * a[col][idx];
                        idx += 1;
                    }
                }

                elim += 1;
            }

            col += 1;
        }

        let mut inverse = [0.0_f64; 16];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                inverse[col * 4 + row] = a[row][col + 4];
                col += 1;
            }

            row += 1;
        }

        Self::from_f64_array(inverse)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.checked_mul_assign.md")]
    pub fn checked_mul_assign(&mut self, rhs: Self) -> Result<(), Mat4Error> {
        let result = *self * rhs;
        *self = result?;
        Ok(())
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.perspective_rh.md")]
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
        {
            return Err(Mat4Error::NonFiniteInput);
        }

        if !(0.0 < fov_y_degrees && fov_y_degrees < 180.0)
            || aspect_ratio <= 0.0
            || near <= 0.0
            || far <= near
        {
            return Err(Mat4Error::InvalidPerspectiveParameters);
        }

        let half_angle = (fov_y_degrees as f64).to_radians() * 0.5;
        let f = 1.0 / half_angle.tan();
        let aspect = aspect_ratio as f64;
        let near = near as f64;
        let far = far as f64;

        let array = [
            f / aspect,
            0.0,
            0.0,
            0.0,
            0.0,
            f,
            0.0,
            0.0,
            0.0,
            0.0,
            far / (near - far),
            -1.0,
            0.0,
            0.0,
            (near * far) / (near - far),
            0.0,
        ];

        Self::from_f64_array(array)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.orthographic_rh.md")]
    pub const fn orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Result<Self, Mat4Error> {
        if !left.is_finite()
            || !right.is_finite()
            || !bottom.is_finite()
            || !top.is_finite()
            || !near.is_finite()
            || !far.is_finite()
        {
            return Err(Mat4Error::NonFiniteInput);
        }

        if left == right || bottom == top || near == far {
            return Err(Mat4Error::InvalidOrthographicParameters);
        }

        let (left, right, bottom, top, near, far) = (
            left as f64,
            right as f64,
            bottom as f64,
            top as f64,
            near as f64,
            far as f64,
        );

        let array = [
            2.0 / (right - left),
            0.0,
            0.0,
            0.0,
            0.0,
            2.0 / (top - bottom),
            0.0,
            0.0,
            0.0,
            0.0,
            -2.0 / (far - near),
            0.0,
            -(right + left) / (right - left),
            -(top + bottom) / (top - bottom),
            -(far + near) / (far - near),
            1.0,
        ];

        Self::from_f64_array(array)
    }

    #[inline]
    #[must_use = "fallible operation: handle the error"]
    #[doc = include_str!("../../docs/api/methods/Mat4.look_at_rh.md")]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Result<Self, Mat4Error> {
        if !eye.is_finite() || !center.is_finite() || !up.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let forward = (center - eye).map_err(|_| Mat4Error::InvalidResult)?;
        let forward_length = forward.length().map_err(|_| Mat4Error::InvalidResult)?;

        if forward_length == 0.0 {
            return Err(Mat4Error::InvalidLookAtParameters);
        }

        let forward = (forward / forward_length).map_err(|_| Mat4Error::InvalidResult)?;

        let side = forward.cross(up).map_err(|_| Mat4Error::InvalidResult)?;
        let side_length = side.length().map_err(|_| Mat4Error::InvalidResult)?;

        if side_length == 0.0 {
            return Err(Mat4Error::InvalidLookAtParameters);
        }

        let side = (side / side_length).map_err(|_| Mat4Error::InvalidResult)?;
        let up_corrected = side.cross(forward).map_err(|_| Mat4Error::InvalidResult)?;

        let side_dot = side.dot(eye).map_err(|_| Mat4Error::InvalidResult)?;
        let up_dot = up_corrected
            .dot(eye)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let forward_dot = forward.dot(eye).map_err(|_| Mat4Error::InvalidResult)?;

        Self::new(
            Vec4::new(side.x, up_corrected.x, -forward.x, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::new(side.y, up_corrected.y, -forward.y, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::new(side.z, up_corrected.z, -forward.z, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::new(-side_dot, -up_dot, forward_dot, 1.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
        )
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Result<Vec4, Mat4Error>;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let columns = self.to_cols_array_2d();
        let v = rhs.to_array();
        let mut result = [0.0_f64; 4];

        for row in 0..4 {
            for col in 0..4 {
                result[row] += columns[col][row] as f64 * v[col] as f64;
            }
        }

        let mut output = [0.0_f32; 4];

        for i in 0..4 {
            if !result[i].is_finite() || result[i].abs() > f32::MAX as f64 {
                return Err(Mat4Error::InvalidResult);
            }

            output[i] = result[i] as f32;
        }

        Vec4::new(output[0], output[1], output[2], output[3]).map_err(|_| Mat4Error::InvalidResult)
    }
}

impl Mul for Mat4 {
    type Output = Result<Self, Mat4Error>;
    fn mul(self, rhs: Self) -> Self::Output {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let a = self.to_cols_array_2d();
        let b = rhs.to_cols_array_2d();
        let mut result = [0.0_f64; 16];

        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0_f64;

                for k in 0..4 {
                    sum += a[k][row] as f64 * b[col][k] as f64;
                }

                result[col * 4 + row] = sum;
            }
        }

        Self::from_f64_array(result)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Mat4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(Error::custom("Mat4 contains a non-finite component"));
        }

        let mut state = serializer.serialize_struct("Mat4", 4)?;

        state.serialize_field("x_axis", &self.x_axis)?;
        state.serialize_field("y_axis", &self.y_axis)?;
        state.serialize_field("z_axis", &self.z_axis)?;
        state.serialize_field("w_axis", &self.w_axis)?;

        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Mat4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMat4 {
            x_axis: Vec4,
            y_axis: Vec4,
            z_axis: Vec4,
            w_axis: Vec4,
        }

        let raw = RawMat4::deserialize(deserializer)?;

        Self::new(raw.x_axis, raw.y_axis, raw.z_axis, raw.w_axis).map_err(serde::de::Error::custom)
    }
}
