use crate::{Mat4Error, Vec3, Vec4};

use super::Mat4;

impl Mat4 {
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
