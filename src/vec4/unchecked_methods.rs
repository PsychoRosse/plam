use crate::Vec3;

use super::Vec4;

impl Vec4 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(values: [f32; 4]) -> Self {
        Self::unchecked_new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value, value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_xyz(self) -> Vec3 {
        Vec3::unchecked_new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    #[inline]
    #[must_use]
    pub fn unchecked_length(self) -> f32 {
        self.unchecked_length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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
        let dw = self.w - rhs.w;

        (dx * dx + dy * dy + dz * dz + dw * dw).sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_lerp(self, rhs: Self, t: f32) -> Self {
        Self::unchecked_new(
            self.x + (rhs.x - self.x) * t,
            self.y + (rhs.y - self.y) * t,
            self.z + (rhs.z - self.z) * t,
            self.w + (rhs.w - self.w) * t,
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_abs(self) -> Self {
        Self::unchecked_new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_min(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x.min(rhs.x),
            self.y.min(rhs.y),
            self.z.min(rhs.z),
            self.w.min(rhs.w),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_max(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x.max(rhs.x),
            self.y.max(rhs.y),
            self.z.max(rhs.z),
            self.w.max(rhs.w),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_distance_squared(self, rhs: Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;
        let dw = self.w - rhs.w;

        dx * dx + dy * dy + dz * dz + dw * dw
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_clamp(self, min: Self, max: Self) -> Self {
        Self::unchecked_new(
            self.x.clamp(min.x, max.x),
            self.y.clamp(min.y, max.y),
            self.z.clamp(min.z, max.z),
            self.w.clamp(min.w, max.w),
        )
    }
}
