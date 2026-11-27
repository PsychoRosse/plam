use crate::Vec3Error;

use super::Vec3;

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(values: [f32; 3]) -> Self {
        Self::unchecked_new(values[0], values[1], values[2])
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    #[must_use]
    pub fn unchecked_length(self) -> f32 {
        self.unchecked_length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_cross(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_normalize(self) -> Self {
        let length = self.unchecked_length();

        self.unchecked_div_scalar(length)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_distance(self, rhs: Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_lerp(self, rhs: Self, t: f32) -> Self {
        Self::unchecked_new(
            self.x + (rhs.x - self.x) * t,
            self.y + (rhs.y - self.y) * t,
            self.z + (rhs.z - self.z) * t,
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_abs(self) -> Self {
        Self::unchecked_new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_min(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_max(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    #[inline]
    pub const fn unchecked_distance_squared(self, rhs: Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;

        dx * dx + dy * dy + dz * dz
    }

    #[inline]
    pub const fn checked_distance_squared(self, rhs: Self) -> Result<f64, Vec3Error> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(Vec3Error::NonFiniteInput);
        }

        let dx = self.x as f64 - rhs.x as f64;
        let dy = self.y as f64 - rhs.y as f64;
        let dz = self.z as f64 - rhs.z as f64;

        Ok(dx * dx + dy * dy + dz * dz)
    }

    #[inline]
    pub const fn unchecked_clamp(self, min: Self, max: Self) -> Self {
        Self::unchecked_new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
        )
    }
}
