use crate::Vec4Error;

use super::Vec4;

impl Vec4 {
    #[inline]
    pub const fn checked_neg(self) -> Result<Self, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.unchecked_neg())
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let result = self.unchecked_add(rhs);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let result = self.unchecked_sub(rhs);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let result = self.unchecked_mul(rhs);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0 || rhs.w == 0.0 {
            return Err(Vec4Error::DivisionByZero);
        }

        let result = self.unchecked_div(rhs);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_mul_scalar(self, scalar: f32) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let result = self.unchecked_mul_scalar(scalar);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_div_scalar(self, scalar: f32) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Vec4Error::DivisionByZero);
        }

        let result = self.unchecked_div_scalar(scalar);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }
}
