use crate::{Vec3, Vec4Error};

use super::Vec4;

impl Vec4 {
    #[inline]
    pub const fn checked_new(x: f32, y: f32, z: f32, w: f32) -> Result<Self, Vec4Error> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() || !w.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(Self::unchecked_new(x, y, z, w))
    }

    #[inline]
    pub const fn checked_from_array(values: [f32; 4]) -> Result<Self, Vec4Error> {
        Self::checked_new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    pub const fn checked_splat(value: f32) -> Result<Self, Vec4Error> {
        Self::checked_new(value, value, value, value)
    }

    #[inline]
    pub const fn checked_xyz(self) -> Result<Vec3, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.unchecked_xyz())
    }

    #[inline]
    pub const fn checked_length_squared(self) -> Result<f64, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        let w = self.w as f64;

        Ok(x * x + y * y + z * z + w * w)
    }

    #[inline]
    pub fn checked_length(self) -> Result<f32, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;

        let scaled_length = (x * x + y * y + z * z + w * w).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_dot(self, rhs: Self) -> Result<f64, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.x as f64 * rhs.x as f64
            + self.y as f64 * rhs.y as f64
            + self.z as f64 * rhs.z as f64
            + self.w as f64 * rhs.w as f64)
    }

    #[inline]
    pub fn checked_normalize(self) -> Result<Self, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Err(Vec4Error::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;
        let length = (x * x + y * y + z * z + w * w).sqrt();

        if !length.is_finite() || length == 0.0 {
            return Err(Vec4Error::InvalidResult);
        }

        let result = Self::unchecked_new(x / length, y / length, z / length, w / length);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_distance(self, rhs: Self) -> Result<f32, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let dx = (self.x as f64 - rhs.x as f64).abs();
        let dy = (self.y as f64 - rhs.y as f64).abs();
        let dz = (self.z as f64 - rhs.z as f64).abs();
        let dw = (self.w as f64 - rhs.w as f64).abs();

        let scale = dx.max(dy).max(dz).max(dw);

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = dx / scale;
        let y = dy / scale;
        let z = dz / scale;
        let w = dw / scale;

        let result = (scale * (x * x + y * y + z * z + w * w).sqrt()) as f32;

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_lerp(self, rhs: Self, t: f32) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() || !t.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let t = t as f64;

        let x = self.x as f64 + (rhs.x as f64 - self.x as f64) * t;

        let y = self.y as f64 + (rhs.y as f64 - self.y as f64) * t;

        let z = self.z as f64 + (rhs.z as f64 - self.z as f64) * t;

        let w = self.w as f64 + (rhs.w as f64 - self.w as f64) * t;

        let result = Self::unchecked_new(x as f32, y as f32, z as f32, w as f32);

        if !result.is_finite() {
            return Err(Vec4Error::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub const fn checked_abs(self) -> Result<Self, Vec4Error> {
        if !self.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.unchecked_abs())
    }

    #[inline]
    pub const fn checked_min(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.unchecked_min(rhs))
    }

    #[inline]
    pub const fn checked_max(self, rhs: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        Ok(self.unchecked_max(rhs))
    }

    #[inline]
    pub const fn checked_distance_squared(self, rhs: Self) -> Result<f64, Vec4Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;
        let dz = self.z as f64 - rhs.z as f64;
        let dw = self.w as f64 - rhs.w as f64;

        Ok(dx * dx + dy * dy + dz * dz + dw * dw)
    }

    #[inline]
    pub const fn checked_clamp(self, min: Self, max: Self) -> Result<Self, Vec4Error> {
        if !self.is_finite() || !min.is_finite() || !max.is_finite() {
            return Err(Vec4Error::NonFiniteInput);
        }

        if min.x > max.x || min.y > max.y || min.z > max.z || min.w > max.w {
            return Err(Vec4Error::InvalidRange);
        }

        Ok(self.unchecked_clamp(min, max))
    }
}
