use crate::QuatError;

use super::Quat;

impl Quat {
    #[inline]
    pub const fn checked_neg(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(self.unchecked_neg())
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let result = self.unchecked_add(rhs);

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let result = self.unchecked_sub(rhs);

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Result<Self, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let result = self.unchecked_mul(rhs);

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_mul_scalar(self, scalar: f32) -> Result<Self, QuatError> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let result = self.unchecked_mul_scalar(scalar);

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_div_scalar(self, scalar: f32) -> Result<Self, QuatError> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(QuatError::DivisionByZero);
        }

        let result = self.unchecked_div_scalar(scalar);

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }
}
