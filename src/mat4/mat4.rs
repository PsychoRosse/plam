use crate::Vec4;

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

    #[inline]
    pub(crate) fn unchecked_from_perspective_parts(
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
    pub(crate) const fn unchecked_from_orthographic_parts(
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

        Mat4::checked_new(raw.x_axis, raw.y_axis, raw.z_axis, raw.w_axis).map_err(DeError::custom)
    }
}
