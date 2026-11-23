use super::Vec2;

impl Vec2 {
    #[inline]
    #[must_use]
    pub fn unchecked_length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    #[must_use]
    pub fn unchecked_length(self) -> f32 {
        self.unchecked_length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
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

        (dx * dx + dy * dy).sqrt()
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_lerp(self, rhs: Self, t: f32) -> Self {
        Self::unchecked_new(self.x + (rhs.x - self.x) * t, self.y + (rhs.y - self.y) * t)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_perpendicular(self) -> Self {
        Self::unchecked_new(-self.y, self.x)
    }
}
