use crate::{QuatError, Vec3};

use super::Quat;

impl Quat {
    #[inline]
    pub const fn checked_dot(self, rhs: Self) -> Result<f64, QuatError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(self.x as f64 * rhs.x as f64
            + self.y as f64 * rhs.y as f64
            + self.z as f64 * rhs.z as f64
            + self.w as f64 * rhs.w as f64)
    }

    #[inline]
    pub const fn checked_length_squared(self) -> Result<f64, QuatError> {
        self.checked_dot(self)
    }

    #[inline]
    pub fn checked_length(self) -> Result<f32, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Ok(0.0);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;

        let scaled_length = (x * x + y * y + z * z + w * w).sqrt();
        let result = scale * scaled_length;

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_normalize(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let scale = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());

        if scale == 0.0 {
            return Err(QuatError::ZeroLength);
        }

        let x = self.x / scale;
        let y = self.y / scale;
        let z = self.z / scale;
        let w = self.w / scale;
        let length = (x * x + y * y + z * z + w * w).sqrt();

        Self::checked_new(x / length, y / length, z / length, w / length)
            .map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    pub const fn checked_conjugate(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        Ok(self.unchecked_conjugate())
    }

    #[inline]
    pub const fn checked_inverse(self) -> Result<Self, QuatError> {
        if !self.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        if self.is_zero() {
            return Err(QuatError::ZeroLength);
        }

        let result = self.unchecked_inverse();

        if !result.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok(result)
    }

    #[inline]
    pub fn checked_is_rotation_eq(self, rhs: Self, epsilon: f32) -> Result<bool, QuatError> {
        if !epsilon.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        if epsilon < 0.0 {
            return Err(QuatError::InvalidResult);
        }

        let a = self.checked_normalize()?;
        let b = rhs.checked_normalize()?;

        let dot = a.checked_dot(b)? as f32;

        Ok((1.0 - dot).abs() <= epsilon)
    }

    #[inline]
    pub fn checked_rotate_vec3(self, vector: Vec3) -> Result<Vec3, QuatError> {
        if !self.is_finite() || !vector.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let q = self.checked_normalize()?;

        let (x, y, z) = (vector.x as f64, vector.y as f64, vector.z as f64);
        let (qx, qy, qz, qw) = (q.x as f64, q.y as f64, q.z as f64, q.w as f64);

        let tx = 2.0 * (qy * z - qz * y);
        let ty = 2.0 * (qz * x - qx * z);
        let tz = 2.0 * (qx * y - qy * x);

        let rx = x + qw * tx + qy * tz - qz * ty;
        let ry = y + qw * ty + qz * tx - qx * tz;
        let rz = z + qw * tz + qx * ty - qy * tx;

        let limit = f32::MAX as f64;

        if !rx.is_finite()
            || !ry.is_finite()
            || !rz.is_finite()
            || rx.abs() > limit
            || ry.abs() > limit
            || rz.abs() > limit
        {
            return Err(QuatError::InvalidResult);
        }

        Vec3::checked_new(rx as f32, ry as f32, rz as f32).map_err(|_| QuatError::InvalidResult)
    }

    #[inline]
    pub fn checked_to_axis_angle(self) -> Result<(Vec3, f32), QuatError> {
        let mut q = self.checked_normalize()?;

        if q.w < 0.0 {
            q = q.checked_neg()?;
        }

        let sin_half = q.x.hypot(q.y).hypot(q.z);

        if !sin_half.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        if sin_half == 0.0 {
            let axis = Vec3::checked_new(1.0, 0.0, 0.0)
                .map_err(|_| QuatError::InvalidResult)?;
            return Ok((axis, 0.0));
        }

        let axis = Vec3::checked_new(q.x / sin_half, q.y / sin_half, q.z / sin_half)
            .map_err(|_| QuatError::InvalidResult)?;

        let angle = 2.0 * sin_half.atan2(q.w).to_degrees();

        if !angle.is_finite() {
            return Err(QuatError::InvalidResult);
        }

        Ok((axis, angle))
    }

    #[inline]
    pub fn checked_slerp(self, end: Self, t: f32) -> Result<Self, QuatError> {
        if !self.is_finite() || !end.is_finite() || !t.is_finite() {
            return Err(QuatError::NonFiniteInput);
        }

        let start = self.checked_normalize()?;
        let mut end = end.checked_normalize()?;
        let mut dot = start.checked_dot(end)? as f32;

        if t == 0.0 {
            return Ok(start);
        }

        if t == 1.0 {
            return Ok(end);
        }

        if dot < 0.0 {
            end = end.checked_neg()?;
            dot = -dot;
        }

        dot = dot.clamp(0.0, 1.0);

        const DOT_THRESHOLD: f32 = 0.9995;

        if dot > DOT_THRESHOLD {
            return start
                .checked_mul_scalar(1.0 - t)?
                .checked_add(end.checked_mul_scalar(t)?)?
                .checked_normalize();
        }

        let theta = dot.acos();
        let sin_theta = theta.sin();

        if !sin_theta.is_finite() || sin_theta == 0.0 {
            return start
                .checked_mul_scalar(1.0 - t)?
                .checked_add(end.checked_mul_scalar(t)?)?
                .checked_normalize();
        }

        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        start
            .checked_mul_scalar(a)?
            .checked_add(end.checked_mul_scalar(b)?)?
            .checked_normalize()
    }
}
