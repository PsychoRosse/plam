use super::Vec2;

impl Vec2 {
    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(values: [f32; 2]) -> Self {
        Self::unchecked_new(values[0], values[1])
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_length_squared(self) -> f32 {
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

    #[inline]
    #[must_use]
    pub const fn unchecked_abs(self) -> Self {
        Self::unchecked_new(self.x.abs(), self.y.abs())
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_min(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_max(self, rhs: Self) -> Self {
        Self::unchecked_new(self.x.max(rhs.x), self.y.max(rhs.y))
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_distance_squared(self, rhs: Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;

        dx * dx + dy * dy
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_clamp(self, min: Self, max: Self) -> Self {
        Self::unchecked_new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}
