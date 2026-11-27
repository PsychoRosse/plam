use crate::Vec3;

use super::Quat;

impl Quat {
    #[inline]
    #[must_use]
    pub const fn unchecked_neg(self) -> Self {
        Self::unchecked_new(-self.x, -self.y, -self.z, -self.w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_add(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }

    #[inline]
    pub const fn unchecked_add_assign(&mut self, rhs: Self) {
        *self = self.unchecked_add(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_sub(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }

    #[inline]
    pub const fn unchecked_sub_assign(&mut self, rhs: Self) {
        *self = self.unchecked_sub(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul(self, rhs: Self) -> Self {
        Self::unchecked_new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }

    #[inline]
    pub const fn unchecked_mul_assign(&mut self, rhs: Self) {
        *self = self.unchecked_mul(rhs);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_vec3(self, rhs: Vec3) -> Vec3 {
        let tx = 2.0 * (self.y * rhs.z - self.z * rhs.y);
        let ty = 2.0 * (self.z * rhs.x - self.x * rhs.z);
        let tz = 2.0 * (self.x * rhs.y - self.y * rhs.x);

        Vec3::unchecked_new(
            rhs.x + self.w * tx + self.y * tz - self.z * ty,
            rhs.y + self.w * ty + self.z * tx - self.x * tz,
            rhs.z + self.w * tz + self.x * ty - self.y * tx,
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }

    #[inline]
    pub const fn unchecked_mul_scalar_assign(&mut self, scalar: f32) {
        *self = self.unchecked_mul_scalar(scalar);
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }

    #[inline]
    pub const fn unchecked_div_scalar_assign(&mut self, scalar: f32) {
        *self = self.unchecked_div_scalar(scalar);
    }
}
