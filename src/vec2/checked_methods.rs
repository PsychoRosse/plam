use crate::Vec2Error;

use super::Vec2;

impl Vec2 {
    #[inline]
    pub const fn checked_new(x: f32, y: f32) -> Result<Self, Vec2Error> {
        if !x.is_finite() || !y.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y))
    }

    #[inline]
    pub const fn checked_from_array(values: [f32; 2]) -> Result<Self, Vec2Error> {
        Self::checked_new(values[0], values[1])
    }

    #[inline]
    pub const fn checked_splat(value: f32) -> Result<Self, Vec2Error> {
        Self::checked_new(value, value)
    }

    #[inline]
    pub const fn checked_length_squared(self) -> Result<f64, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let x = self.x as f64;
        let y = self.y as f64;

        Ok(x * x + y * y)
    }

    #[inline]
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
    pub const fn checked_dot(self, rhs: Self) -> Result<f64, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.x as f64 * rhs.x as f64 + self.y as f64 * rhs.y as f64)
    }

    #[inline]
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
    pub fn checked_distance(self, rhs: Self) -> Result<f32, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;

        let result = (dx * dx + dy * dy).sqrt() as f32;

        if !result.is_finite() {
            return Err(Vec2Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_lerp(self, rhs: Self, t: f32) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() || !t.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = self.x as f64 + (rhs.x as f64 - self.x as f64) * t;

        let y = self.y as f64 + (rhs.y as f64 - self.y as f64) * t;

        match Self::checked_new(x as f32, y as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec2Error::InvalidResult),
        }
    }

    #[inline]
    pub const fn checked_perpendicular(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(-self.y, self.x))
    }

    #[inline]
    pub const fn checked_abs(self) -> Result<Self, Vec2Error> {
        if !self.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.unchecked_abs())
    }

    #[inline]
    pub const fn checked_min(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.unchecked_min(rhs))
    }

    #[inline]
    pub const fn checked_max(self, rhs: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.unchecked_max(rhs))
    }

    #[inline]
    pub const fn checked_cross(self, rhs: Self) -> Result<f64, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        Ok(self.x as f64 * rhs.y as f64 - self.y as f64 * rhs.x as f64)
    }

    #[inline]
    pub const fn checked_distance_squared(self, rhs: Self) -> Result<f64, Vec2Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;

        Ok(dx * dx + dy * dy)
    }

    #[inline]
    pub const fn checked_clamp(self, min: Self, max: Self) -> Result<Self, Vec2Error> {
        if !self.is_finite() || !min.is_finite() || !max.is_finite() {
            return Err(Vec2Error::NonFiniteInput);
        }

        if min.x > max.x || min.y > max.y {
            return Err(Vec2Error::InvalidRange);
        }

        Ok(self.unchecked_clamp(min, max))
    }
}
