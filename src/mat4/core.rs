use crate::{Mat4Error, Quat, Vec3, Vec4};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize, Serializer,
    de::Error as DeError,
    ser::{Error as SerError, SerializeStruct},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub(crate) x_axis: Vec4,
    pub(crate) y_axis: Vec4,
    pub(crate) z_axis: Vec4,
    pub(crate) w_axis: Vec4,
}

impl Default for Mat4 {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mat4 {
    pub const IDENTITY: Self = Self::unchecked_new(
        Vec4::unchecked_new(1.0, 0.0, 0.0, 0.0),
        Vec4::unchecked_new(0.0, 1.0, 0.0, 0.0),
        Vec4::unchecked_new(0.0, 0.0, 1.0, 0.0),
        Vec4::unchecked_new(0.0, 0.0, 0.0, 1.0),
    );
}

impl Mat4 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(
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
    #[must_use]
    pub const fn unchecked_from_cols(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Self {
        Self::unchecked_new(x_axis, y_axis, z_axis, w_axis)
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
    #[must_use]
    pub const fn unchecked_from_array(array: [f32; 16]) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(array[0], array[1], array[2], array[3]),
            Vec4::unchecked_new(array[4], array[5], array[6], array[7]),
            Vec4::unchecked_new(array[8], array[9], array[10], array[11]),
            Vec4::unchecked_new(array[12], array[13], array[14], array[15]),
        )
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
    #[must_use]
    pub(crate) const fn unchecked_from_f64_array(array: [f64; 16]) -> Self {
        let mut values = [0.0_f32; 16];
        let mut index = 0;

        while index < 16 {
            values[index] = array[index] as f32;

            index += 1;
        }

        Self::unchecked_from_array(values)
    }

    #[inline]
    #[must_use]
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
    pub const fn is_finite(self) -> bool {
        self.x_axis.is_finite()
            && self.y_axis.is_finite()
            && self.z_axis.is_finite()
            && self.w_axis.is_finite()
    }

    #[inline]
    #[must_use]
    pub const fn is_approx_eq(self, rhs: Self, epsilon: f32) -> bool {
        if !epsilon.is_finite() || epsilon < 0.0 {
            return false;
        }

        let a = self.to_array();
        let b = rhs.to_array();

        let mut i = 0;

        while i < 16 {
            if (a[i] - b[i]).abs() > epsilon {
                return false;
            }

            i += 1;
        }

        true
    }
}

impl Mat4 {
    #[inline]
    pub const fn checked_from_translation(translation: Vec3) -> Result<Self, Mat4Error> {
        if !translation.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_from_translation(translation))
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_translation(translation: Vec3) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(1.0, 0.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 1.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 0.0, 1.0, 0.0),
            Vec4::unchecked_new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    #[inline]
    pub const fn checked_from_scale(scale: Vec3) -> Result<Self, Mat4Error> {
        if !scale.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_from_scale(scale))
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_scale(scale: Vec3) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(scale.x, 0.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, scale.y, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 0.0, scale.z, 0.0),
            Vec4::unchecked_new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn checked_from_quat(rotation: Quat) -> Result<Self, Mat4Error> {
        let q = rotation
            .checked_normalize()
            .map_err(|_| Mat4Error::InvalidQuaternion)?;

        Ok(Self::unchecked_from_quat(q))
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_quat(rotation: Quat) -> Self {
        let q = rotation.unchecked_normalize();

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

        Self::unchecked_new(
            Vec4::unchecked_new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
            Vec4::unchecked_new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
            Vec4::unchecked_new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
            Vec4::unchecked_new(0.0, 0.0, 0.0, 1.0),
        )
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
    #[must_use]
    pub fn unchecked_from_scale_rotation_translation(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        let rotation = Self::unchecked_from_quat(rotation);

        Self::unchecked_new(
            Vec4::unchecked_new(
                rotation.x_axis.x * scale.x,
                rotation.x_axis.y * scale.x,
                rotation.x_axis.z * scale.x,
                0.0,
            ),
            Vec4::unchecked_new(
                rotation.y_axis.x * scale.y,
                rotation.y_axis.y * scale.y,
                rotation.y_axis.z * scale.y,
                0.0,
            ),
            Vec4::unchecked_new(
                rotation.z_axis.x * scale.z,
                rotation.z_axis.y * scale.z,
                rotation.z_axis.z * scale.z,
                0.0,
            ),
            Vec4::unchecked_new(translation.x, translation.y, translation.z, 1.0),
        )
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
    #[must_use]
    pub fn unchecked_perspective_rh(
        fov_y_degrees: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self::unchecked_from_perspective_parts(fov_y_degrees, aspect_ratio, near, far)
    }

    #[inline]
    fn unchecked_from_perspective_parts(
        fov_y_degrees: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let half_angle = (fov_y_degrees as f64).to_radians() * 0.5;
        let f = 1.0 / half_angle.tan();
        let aspect = aspect_ratio as f64;
        let near = near as f64;
        let far = far as f64;

        Self::unchecked_from_f64_array([
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
        ])
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
    #[must_use]
    pub const fn unchecked_orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self::unchecked_from_orthographic_parts(left, right, bottom, top, near, far)
    }

    #[inline]
    const fn unchecked_from_orthographic_parts(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let (left, right, bottom, top, near, far) = (
            left as f64,
            right as f64,
            bottom as f64,
            top as f64,
            near as f64,
            far as f64,
        );

        Self::unchecked_from_f64_array([
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
        ])
    }

    #[inline]
    pub fn checked_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Result<Self, Mat4Error> {
        if !eye.is_finite() || !center.is_finite() || !up.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let forward = center.checked_sub(eye).map_err(|_| Mat4Error::InvalidResult)?;
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
        let side_length = side.checked_length().map_err(|_| Mat4Error::InvalidResult)?;

        if side_length == 0.0 {
            return Err(Mat4Error::InvalidLookAtParameters);
        }

        let side = side
            .checked_div_scalar(side_length)
            .map_err(|_| Mat4Error::InvalidResult)?;
        let up_corrected = side
            .checked_cross(forward)
            .map_err(|_| Mat4Error::InvalidResult)?;

        let side_dot = side.checked_dot(eye).map_err(|_| Mat4Error::InvalidResult)?;
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
    #[must_use]
    pub fn unchecked_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let forward = center.unchecked_sub(eye);
        let forward_length = forward.unchecked_length();
        let forward = forward.unchecked_div_scalar(forward_length);

        let side = forward.unchecked_cross(up);
        let side_length = side.unchecked_length();
        let side = side.unchecked_div_scalar(side_length);
        let up_corrected = side.unchecked_cross(forward);

        let side_dot = side.unchecked_dot(eye);
        let up_dot = up_corrected.unchecked_dot(eye);
        let forward_dot = forward.unchecked_dot(eye);

        Self::unchecked_new(
            Vec4::unchecked_new(side.x, up_corrected.x, -forward.x, 0.0),
            Vec4::unchecked_new(side.y, up_corrected.y, -forward.y, 0.0),
            Vec4::unchecked_new(side.z, up_corrected.z, -forward.z, 0.0),
            Vec4::unchecked_new(-side_dot, -up_dot, forward_dot, 1.0),
        )
    }
}

#[cfg(feature = "serde")]
impl Serialize for Mat4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if !self.is_finite() {
            return Err(SerError::custom("Mat4 contains a non-finite component"));
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
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMat4 {
            x_axis: Vec4,
            y_axis: Vec4,
            z_axis: Vec4,
            w_axis: Vec4,
        }

        let raw = RawMat4::deserialize(deserializer)?;

        Mat4::checked_new(raw.x_axis, raw.y_axis, raw.z_axis, raw.w_axis)
            .map_err(DeError::custom)
    }
}
