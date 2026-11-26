use crate::Vec3;

use super::Quat;

use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

impl Neg for Quat {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.unchecked_neg()
    }
}

impl Add for Quat {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.unchecked_add(rhs)
    }
}

impl Sub for Quat {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.unchecked_sub(rhs)
    }
}

impl Mul for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.unchecked_mul(rhs)
    }
}

impl MulAssign for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.unchecked_mul(rhs);
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.unchecked_mul_scalar(rhs)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.unchecked_mul_vec3(rhs)
    }
}

impl Mul<Quat> for f32 {
    type Output = Quat;

    #[inline]
    fn mul(self, rhs: Quat) -> Self::Output {
        rhs.unchecked_mul_scalar(self)
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.unchecked_div_scalar(rhs)
    }
}
