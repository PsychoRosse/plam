use crate::Vec3Error;

use super::Vec3;

impl Vec3 {
    #[inline]
    pub const fn checked_new(x: f32, y: f32, z: f32) -> Result<Self, Vec3Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y, z))
    }

    #[inline]
    pub const fn checked_splat(value: f32) -> Result<Self, Vec3Error> {
        Self::checked_new(value, value, value)
    }

    #[inline]
    pub const fn checked_length_squared(self) -> Result<f64, Vec3Error> {
        if !self.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;

        Ok(x * x + y * y + z * z)
    }

    #[inline]
    pub fn checked_length(self) -> Result<f32, Vec3Error> {
        if !self.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let scale = self.x.abs().max(self.y.abs()).max(self.z.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;

        let scaled_length = (x * x + y * y + z * z).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_dot(self, rhs: Self) -> Result<f64, Vec3Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        Ok(self.x as f64 * rhs.x as f64
            + self.y as f64 * rhs.y as f64
            + self.z as f64 * rhs.z as f64)
    }

    #[inline]
    pub const fn checked_cross(self, rhs: Self) -> Result<Self, Vec3Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let x = self.y as f64 * rhs.z as f64 - self.z as f64 * rhs.y as f64;
        let y = self.z as f64 * rhs.x as f64 - self.x as f64 * rhs.z as f64;
        let z = self.x as f64 * rhs.y as f64 - self.y as f64 * rhs.x as f64;

        match Self::checked_new(x as f32, y as f32, z as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec3Error::InvalidResult),
        }
    }

    #[inline]
    pub fn checked_normalize(self) -> Result<Self, Vec3Error> {
        if !self.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let scale = self.x.abs().max(self.y.abs()).max(self.z.abs());

        if scale == 0.0 {
            return Err(Vec3Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let length = (x * x + y * y + z * z).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec3Error::InvalidResult);
        }

        let result = Self::unchecked_new(x / length, y / length, z / length);

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_distance(self, rhs: Self) -> Result<f32, Vec3Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;
        let dz = self.z as f64 - rhs.z as f64;

        let result = (dx * dx + dy * dy + dz * dz).sqrt() as f32;

        if !result.is_finite() {
            return Err(Vec3Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_lerp(self, rhs: Self, t: f32) -> Result<Self, Vec3Error> {
        if !self.is_finite() || !rhs.is_finite() || !t.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = self.x as f64 + (rhs.x as f64 - self.x as f64) * t;

        let y = self.y as f64 + (rhs.y as f64 - self.y as f64) * t;

        let z = self.z as f64 + (rhs.z as f64 - self.z as f64) * t;

        match Self::checked_new(x as f32, y as f32, z as f32) {
            Ok(value) => Ok(value),
            Err(_) => Err(Vec3Error::InvalidResult),
        }
    }
}
