use super::Mat4;

use crate::Vec4;

use std::ops::Mul;

impl Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.unchecked_mul(rhs)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        self.unchecked_mul_vec4(rhs)
    }
}
