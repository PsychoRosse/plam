use crate::Vec2Error;

use super::Vec2;

impl Vec2 {
    #[inline]
    #[must_use]
    pub const fn checked_length_squared(self) -> Result<f32, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let x = self.x as f64;
        let y = self.y as f64;

        let result = (x * x + y * y) as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub fn checked_length(self) -> Result<f32, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;

        let scaled_length = (x * x + y * y).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub const fn checked_dot(self, other: Self) -> Result<f32, Vec2Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let result = self.x as f64 * other.x as f64 + self.y as f64 * other.y as f64;

        let result = result as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub fn checked_normalize(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let scale = self.x.abs().max(self.y.abs());

        if scale == 0.0 {
            return Err(Vec2Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let length = (x * x + y * y).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec2Error::InvalidResult);
        }

        let result = Self::unchecked_new(x / length, y / length);

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub fn checked_distance(self, other: Self) -> Result<f32, Vec2Error> {
        if !self.is_finite() || !other.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;

        let result = (dx * dx + dy * dy).sqrt() as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    #[must_use]
    pub const fn checked_lerp(self, other: Self, t: f32) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !other.is_finite() || !t.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = self.x as f64 + (other.x as f64 - self.x as f64) * t;

        let y = self.y as f64 + (other.y as f64 - self.y as f64) * t;

        match Self::checked_new(x as f32, y as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec2Error::InvalidResult),
        }
    }

    #[inline]
    #[must_use]
    pub const fn checked_perpendicular(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(-self.y, self.x))
    }
}
