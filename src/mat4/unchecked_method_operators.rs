use crate::Vec4;

use super::Mat4;

impl Mat4 {
    #[inline]
    #[must_use]
    pub const fn unchecked_neg(self) -> Self {
        Self::unchecked_new(
            self.x_axis.unchecked_neg(),
            self.y_axis.unchecked_neg(),
            self.z_axis.unchecked_neg(),
            self.w_axis.unchecked_neg(),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_add(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x_axis.unchecked_add(rhs.x_axis),
            self.y_axis.unchecked_add(rhs.y_axis),
            self.z_axis.unchecked_add(rhs.z_axis),
            self.w_axis.unchecked_add(rhs.w_axis),
        )
    }

    #[inline]
    pub const fn unchecked_add_assign(&mut self, rhs: Self) {
        *self = self.unchecked_add(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_sub(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x_axis.unchecked_sub(rhs.x_axis),
            self.y_axis.unchecked_sub(rhs.y_axis),
            self.z_axis.unchecked_sub(rhs.z_axis),
            self.w_axis.unchecked_sub(rhs.w_axis),
        )
    }

    #[inline]
    pub const fn unchecked_sub_assign(&mut self, rhs: Self) {
        *self = self.unchecked_sub(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul(self, rhs: Self) -> Self {
        let a = self.to_cols_array_2d();
        let b = rhs.to_cols_array_2d();
        let mut result = [0.0_f64; 16];

        let mut col = 0;

        while col < 4 {
            let mut row = 0;

            while row < 4 {
                let mut sum = 0.0_f64;
                let mut k = 0;

                while k < 4 {
                    sum += a[k][row] as f64 * b[col][k] as f64;
                    k += 1;
                }

                result[col * 4 + row] = sum;
                row += 1;
            }

            col += 1;
        }

        Self::unchecked_from_f64_array(result)
    }

    #[inline]
    pub const fn unchecked_mul_assign(&mut self, rhs: Self) {
        *self = self.unchecked_mul(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_vec4(self, rhs: Vec4) -> Vec4 {
        let columns = self.to_cols_array_2d();
        let v = rhs.to_array();
        let mut result = [0.0_f64; 4];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                result[row] += columns[col][row] as f64 * v[col] as f64;
                col += 1;
            }

            row += 1;
        }

        Vec4::unchecked_new(
            result[0] as f32,
            result[1] as f32,
            result[2] as f32,
            result[3] as f32,
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(
            self.x_axis.unchecked_mul_scalar(scalar),
            self.y_axis.unchecked_mul_scalar(scalar),
            self.z_axis.unchecked_mul_scalar(scalar),
            self.w_axis.unchecked_mul_scalar(scalar),
        )
    }

    #[inline]
    pub const fn unchecked_mul_scalar_assign(&mut self, scalar: f32) {
        *self = self.unchecked_mul_scalar(scalar);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(
            self.x_axis.unchecked_div_scalar(scalar),
            self.y_axis.unchecked_div_scalar(scalar),
            self.z_axis.unchecked_div_scalar(scalar),
            self.w_axis.unchecked_div_scalar(scalar),
        )
    }

    #[inline]
    pub const fn unchecked_div_scalar_assign(&mut self, scalar: f32) {
        *self = self.unchecked_div_scalar(scalar);
    }
}
