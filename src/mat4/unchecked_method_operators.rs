use crate::Vec4;

use super::Mat4;

impl Mat4 {
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
    pub fn unchecked_mul_assign(&mut self, rhs: Self) {
        *self = self.unchecked_mul(rhs);
    }
}
