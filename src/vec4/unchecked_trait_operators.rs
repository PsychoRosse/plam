use super::Vec4;

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Neg for Vec4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.unchecked_neg()
    }
}

impl Add for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.unchecked_add(rhs)
    }
}

impl Sub for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.unchecked_sub(rhs)
    }
}

impl Mul for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.unchecked_mul(rhs)
    }
}

impl Div for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.unchecked_div(rhs)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.unchecked_mul_scalar(rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs.unchecked_mul_scalar(self)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.unchecked_div_scalar(rhs)
    }
}
