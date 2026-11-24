use crate::{Mat4Error, Quat, Vec3, Vec4};

use super::Mat4;

impl Mat4 {
    #[inline]
    pub fn checked_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Result<Self, Mat4Error> {
        if !eye.is_finite() || !center.is_finite() || !up.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let forward = center
            .checked_sub(eye)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let forward_length = forward
            .checked_length()
            .map_err(|_| Mat4Error::InvalidResult)?;

        if forward_length == 0.0 {
            return Err(Mat4Error::InvalidLookAtParameters);
        }

        let forward = forward
            .checked_div_scalar(forward_length)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let side = forward
            .checked_cross(up)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let side_length = side
            .checked_length()
            .map_err(|_| Mat4Error::InvalidResult)?;

        if side_length == 0.0 {
            return Err(Mat4Error::InvalidLookAtParameters);
        }

        let side = side
            .checked_div_scalar(side_length)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let up_corrected = side
            .checked_cross(forward)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let side_dot = side
            .checked_dot(eye)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let up_dot = up_corrected
            .checked_dot(eye)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let forward_dot = forward
            .checked_dot(eye)
            .map_err(|_| Mat4Error::InvalidResult)?;

        Self::checked_new(
            Vec4::checked_new(side.x, up_corrected.x, -forward.x, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::checked_new(side.y, up_corrected.y, -forward.y, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::checked_new(side.z, up_corrected.z, -forward.z, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
            Vec4::checked_new(
                -(side_dot as f32),
                -(up_dot as f32),
                forward_dot as f32,
                1.0,
            )
            .map_err(|_| Mat4Error::InvalidResult)?,
        )
    }

    #[inline]
    pub const fn checked_orthographic_rh(
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

        Ok(Self::unchecked_from_orthographic_parts(
            left, right, bottom, top, near, far,
        ))
    }

    #[inline]
    pub fn checked_perspective_rh(
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

        Ok(Self::unchecked_from_perspective_parts(
            fov_y_degrees,
            aspect_ratio,
            near,
            far,
        ))
    }

    #[inline]
    pub(crate) const fn checked_from_f64_array(array: [f64; 16]) -> Result<Self, Mat4Error> {
        let mut values = [0.0_f32; 16];
        let mut index = 0;

        while index < 16 {
            let value = array[index];

            if !value.is_finite() || value.abs() > f32::MAX as f64 {
                return Err(Mat4Error::InvalidResult);
            }

            values[index] = value as f32;

            index += 1;
        }

        Ok(Self::unchecked_from_array(values))
    }

    #[inline]
    pub const fn checked_from_translation(translation: Vec3) -> Result<Self, Mat4Error> {
        if !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_from_translation(translation))
    }

    #[inline]
    pub const fn checked_from_scale(scale: Vec3) -> Result<Self, Mat4Error> {
        if !scale.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_from_scale(scale))
    }

    #[inline]
    pub fn checked_from_quat(rotation: Quat) -> Result<Self, Mat4Error> {
        let q = rotation
            .checked_normalize()
            .map_err(|_| Mat4Error::InvalidQuaternion)?;

        Ok(Self::unchecked_from_quat(q))
    }

    #[inline]
    pub fn checked_from_scale_rotation_translation(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Result<Self, Mat4Error> {
        if !scale.is_finite() || !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let rotation = Self::checked_from_quat(rotation)?;

        let x_axis = Vec4::checked_new(
            rotation.x_axis.x * scale.x,
            rotation.x_axis.y * scale.x,
            rotation.x_axis.z * scale.x,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let y_axis = Vec4::checked_new(
            rotation.y_axis.x * scale.y,
            rotation.y_axis.y * scale.y,
            rotation.y_axis.z * scale.y,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let z_axis = Vec4::checked_new(
            rotation.z_axis.x * scale.z,
            rotation.z_axis.y * scale.z,
            rotation.z_axis.z * scale.z,
            0.0,
        )
        .map_err(|_| Mat4Error::InvalidResult)?;

        let w_axis = Vec4::checked_new(translation.x, translation.y, translation.z, 1.0)
            .map_err(|_| Mat4Error::InvalidResult)?;

        Self::checked_new(x_axis, y_axis, z_axis, w_axis).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    pub const fn checked_from_array(array: [f32; 16]) -> Result<Self, Mat4Error> {
        let mut index = 0;

        while index < 16 {
            if !array[index].is_finite() {
                return Err(Mat4Error::NonFiniteInput);
            }

            index += 1;
        }

        Ok(Self::unchecked_from_array(array))
    }

    #[inline]
    pub const fn checked_new(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Result<Self, Mat4Error> {
        if !x_axis.is_finite() || !y_axis.is_finite() || !z_axis.is_finite() || !w_axis.is_finite()
        {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x_axis, y_axis, z_axis, w_axis))
    }

    #[inline]
    pub const fn checked_from_cols(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Result<Self, Mat4Error> {
        Self::checked_new(x_axis, y_axis, z_axis, w_axis)
    }

    #[inline]
    pub const fn checked_transpose(self) -> Result<Self, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(self.unchecked_transpose())
    }

    #[inline]
    pub const fn checked_determinant(self) -> Result<f64, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(self.unchecked_determinant_f64())
    }

    #[inline]
    pub const fn checked_inverse(self) -> Result<Self, Mat4Error> {
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

        Self::checked_from_f64_array(inverse)
    }

    #[inline]
    pub fn checked_transform_point3(self, point: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !point.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.checked_mul_vec4(
            Vec4::checked_new(point.x, point.y, point.z, 1.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
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

        Vec3::checked_new(x as f32, y as f32, z as f32).map_err(|_| Mat4Error::InvalidResult)
    }

    #[inline]
    pub fn checked_transform_vector3(self, vector: Vec3) -> Result<Vec3, Mat4Error> {
        if !self.is_finite() || !vector.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.checked_mul_vec4(
            Vec4::checked_new(vector.x, vector.y, vector.z, 0.0)
                .map_err(|_| Mat4Error::InvalidResult)?,
        )?;

        Vec3::checked_new(result.x, result.y, result.z).map_err(|_| Mat4Error::InvalidResult)
    }
}
