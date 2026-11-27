use crate::{Mat4Error, Vec4};

use super::Mat4;

impl Mat4 {
    #[inline]
    pub const fn checked_neg(self) -> Result<Self, Mat4Error> {
        if !self.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        Ok(self.unchecked_neg())
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, Mat4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.unchecked_add(rhs);

        if !result.is_finite() {
            return Err(Mat4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_add_assign(&mut self, rhs: Self) -> Result<(), Mat4Error> {
        *self = self.checked_add(rhs)?;

        Ok(())
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, Mat4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.unchecked_sub(rhs);

        if !result.is_finite() {
            return Err(Mat4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_sub_assign(&mut self, rhs: Self) -> Result<(), Mat4Error> {
        *self = self.checked_sub(rhs)?;

        Ok(())
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Result<Self, Mat4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

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

        Self::checked_from_f64_array(result)
    }

    #[inline]
    pub const fn checked_mul_vec4(self, rhs: Vec4) -> Result<Vec4, Mat4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

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

        let mut index = 0;

        while index < 4 {
            if result[index].abs() > f32::MAX as f64 {
                return Err(Mat4Error::InvalidResult);
            }

            index += 1;
        }

        match Vec4::checked_new(
            result[0] as f32,
            result[1] as f32,
            result[2] as f32,
            result[3] as f32,
        ) {
            Ok(value) => Ok(value),
            Err(_) => Err(Mat4Error::InvalidResult),
        }
    }

    #[inline]
    pub fn checked_mul_assign(&mut self, rhs: Self) -> Result<(), Mat4Error> {
        *self = self.checked_mul(rhs)?;

        Ok(())
    }

    #[inline]
    pub const fn checked_mul_scalar(self, scalar: f32) -> Result<Self, Mat4Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        let result = self.unchecked_mul_scalar(scalar);

        if !result.is_finite() {
            return Err(Mat4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_mul_scalar_assign(&mut self, scalar: f32) -> Result<(), Mat4Error> {
        *self = self.checked_mul_scalar(scalar)?;

        Ok(())
    }

    #[inline]
    pub const fn checked_div_scalar(self, scalar: f32) -> Result<Self, Mat4Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Mat4Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Mat4Error::DivisionByZero);
        }

        let result = self.unchecked_div_scalar(scalar);

        if !result.is_finite() {
            return Err(Mat4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_div_scalar_assign(&mut self, scalar: f32) -> Result<(), Mat4Error> {
        *self = self.checked_div_scalar(scalar)?;

        Ok(())
    }

    #[inline]
    pub fn checked_neg_assign(&mut self) -> Result<(), Mat4Error> {
        *self = self.checked_neg()?;
        Ok(())
    }
}
