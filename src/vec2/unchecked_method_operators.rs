use super::Vec2;

impl Vec2 {
    #[inline]
    #[must_use]
    pub const fn unchecked_neg(self) -> Self {
        Self::unchecked_new(-self.x, -self.y)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_add(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x + rhs.x, self.y + rhs.y)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_sub(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x - rhs.x, self.y - rhs.y)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x * rhs.x, self.y * rhs.y)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x / rhs.x, self.y / rhs.y)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_mul_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(self.x * scalar, self.y * scalar)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_div_scalar(self, scalar: f32) -> Self {
        Self::unchecked_new(self.x / scalar, self.y / scalar)
    }
}
