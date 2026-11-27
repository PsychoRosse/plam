use crate::Vec2Error;

use super::Vec2;

impl Vec2 {
    #[inline]
    pub const fn checked_neg(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.unchecked_neg())
    }

    #[inline]
    pub const fn checked_add(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.unchecked_add(rhs);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_add_assign(&mut self, rhs: Self) -> Result<(), Vec2Error> {
        *self = self.checked_add(rhs)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_sub(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.unchecked_sub(rhs);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_sub_assign(&mut self, rhs: Self) -> Result<(), Vec2Error> {
        *self = self.checked_sub(rhs)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_mul(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.unchecked_mul(rhs);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_mul_assign(&mut self, rhs: Self) -> Result<(), Vec2Error> {
        *self = self.checked_mul(rhs)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_div(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        if rhs.x == 0.0 || rhs.y == 0.0 {
            return Err(Vec2Error::DivisionByZero);
        }

        let result = self.unchecked_div(rhs);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_div_assign(&mut self, rhs: Self) -> Result<(), Vec2Error> {
        *self = self.checked_div(rhs)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_mul_scalar(self, scalar: f32) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.unchecked_mul_scalar(scalar);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_mul_scalar_assign(&mut self, scalar: f32) -> Result<(), Vec2Error> {
        *self = self.checked_mul_scalar(scalar)?;
        Ok(())
    }

    #[inline]
    pub const fn checked_div_scalar(self, scalar: f32) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !scalar.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        if scalar == 0.0 {
            return Err(Vec2Error::DivisionByZero);
        }

        let result = self.unchecked_div_scalar(scalar);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_div_scalar_assign(&mut self, scalar: f32) -> Result<(), Vec2Error> {
        *self = self.checked_div_scalar(scalar)?;
        Ok(())
    }

    #[inline]
    pub fn checked_neg_assign(&mut self) -> Result<(), Vec2Error> {
        *self = self.checked_neg()?;
        Ok(())
    }
}
