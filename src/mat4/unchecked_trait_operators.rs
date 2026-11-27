use crate::Vec4;

use super::Mat4;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Neg for Mat4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.unchecked_neg()
    }
}

impl Add for Mat4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.unchecked_add(rhs)
    }
}

impl AddAssign for Mat4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.unchecked_add(rhs);
    }
}

impl Sub for Mat4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.unchecked_sub(rhs)
    }
}

impl SubAssign for Mat4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.unchecked_sub(rhs);
    }
}

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.unchecked_mul(rhs)
    }
}

impl MulAssign for Mat4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.unchecked_mul(rhs);
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        self.unchecked_mul_vec4(rhs)
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.unchecked_mul_scalar(rhs)
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    #[inline]
    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs.unchecked_mul_scalar(self)
    }
}

impl MulAssign<f32> for Mat4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = self.unchecked_mul_scalar(rhs);
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.unchecked_div_scalar(rhs)
    }
}

impl DivAssign<f32> for Mat4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = self.unchecked_div_scalar(rhs);
    }
}
