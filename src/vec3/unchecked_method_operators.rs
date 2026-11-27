use super::Vec3;

impl Vec3 {
    #[inline]
    #[must_use]
    pub const fn unchecked_neg(self) -> Self {
        Self::unchecked_new(-self.x, -self.y, -self.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_add(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    #[inline]
    pub fn unchecked_add_assign(&mut self, rhs: Self) {
        *self = Self::unchecked_new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_sub(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    #[inline]
    pub fn unchecked_sub_assign(&mut self, rhs: Self) {
        *self = Self::unchecked_new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    #[inline]
    pub fn unchecked_mul_assign(&mut self, rhs: Self) {
        *self = Self::unchecked_new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }

    #[inline]
    pub fn unchecked_div_assign(&mut self, rhs: Self) {
        *self = Self::unchecked_new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    #[inline]
    pub fn unchecked_mul_scalar_assign(&mut self, scalar: f32) {
        *self = Self::unchecked_new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    #[inline]
    pub fn unchecked_div_scalar_assign(&mut self, scalar: f32) {
        *self = Self::unchecked_new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
