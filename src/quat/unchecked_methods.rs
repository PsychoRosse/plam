use crate::Vec3;

use super::Quat;

impl Quat {
    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(values: [f32; 4]) -> Self {
        Self::unchecked_new(values[0], values[1], values[2], values[3])
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_splat(value: f32) -> Self {
        Self::unchecked_new(value, value, value, value)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
        let x = x_degrees.rem_euclid(360.0).to_radians();
        let y = y_degrees.rem_euclid(360.0).to_radians();
        let z = z_degrees.rem_euclid(360.0).to_radians();

        let (sin_x, cos_x) = (x * 0.5).sin_cos();
        let (sin_y, cos_y) = (y * 0.5).sin_cos();
        let (sin_z, cos_z) = (z * 0.5).sin_cos();

        Self::unchecked_new(
            sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z + sin_x * sin_y * cos_z,
            cos_x * cos_y * cos_z - sin_x * sin_y * sin_z,
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        let scale = axis.x.abs().max(axis.y.abs()).max(axis.z.abs());

        let x = axis.x / scale;
        let y = axis.y / scale;
        let z = axis.z / scale;
        let length = (x * x + y * y + z * z).sqrt();

        let half_angle = degrees.rem_euclid(360.0).to_radians() * 0.5;
        let (sin, cos) = half_angle.sin_cos();

        Self::unchecked_new(
            (x / length) * sin,
            (y / length) * sin,
            (z / length) * sin,
            cos,
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_scaled_axis(scaled_axis: Vec3) -> Self {
        let scale = scaled_axis
            .x
            .abs()
            .max(scaled_axis.y.abs())
            .max(scaled_axis.z.abs());

        if scale == 0.0 {
            return Self::IDENTITY;
        }

        let x = scaled_axis.x / scale;
        let y = scaled_axis.y / scale;
        let z = scaled_axis.z / scale;

        let scaled_length = (x * x + y * y + z * z).sqrt();

        let angle = (scale as f64) * (scaled_length as f64);
        let half_angle = angle.rem_euclid(360.0).to_radians() * 0.5;

        let (sin, cos) = half_angle.sin_cos();

        let sin = sin as f32;
        let cos = cos as f32;

        Self::unchecked_new(
            (x / scaled_length) * sin,
            (y / scaled_length) * sin,
            (z / scaled_length) * sin,
            cos,
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_xyz(self) -> Vec3 {
        Vec3::unchecked_new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_length_squared(self) -> f32 {
        self.unchecked_dot(self)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_length(self) -> f32 {
        self.unchecked_length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn unchecked_normalize(self) -> Self {
        let length = self.unchecked_length();

        self.unchecked_div_scalar(length)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_conjugate(self) -> Self {
        Self::unchecked_new(-self.x, -self.y, -self.z, self.w)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_inverse(self) -> Self {
        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        let w = self.w as f64;

        let norm_squared = x * x + y * y + z * z + w * w;

        Self::unchecked_new(
            (-x / norm_squared) as f32,
            (-y / norm_squared) as f32,
            (-z / norm_squared) as f32,
            (w / norm_squared) as f32,
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_is_rotation_eq(self, rhs: Self, epsilon: f32) -> bool {
        let a = self.unchecked_normalize();
        let b = rhs.unchecked_normalize();

        let dot = a.unchecked_dot(b).abs();

        (1.0 - dot).abs() <= epsilon
    }

    #[inline]
    #[must_use]
    pub fn unchecked_rotate_vec3(self, vector: Vec3) -> Vec3 {
        let q = self.unchecked_normalize();

        let (x, y, z) = (vector.x as f64, vector.y as f64, vector.z as f64);
        let (qx, qy, qz, qw) = (q.x as f64, q.y as f64, q.z as f64, q.w as f64);

        let tx = 2.0 * (qy * z - qz * y);
        let ty = 2.0 * (qz * x - qx * z);
        let tz = 2.0 * (qx * y - qy * x);

        let rx = x + qw * tx + qy * tz - qz * ty;
        let ry = y + qw * ty + qz * tx - qx * tz;
        let rz = z + qw * tz + qx * ty - qy * tx;

        Vec3::unchecked_new(rx as f32, ry as f32, rz as f32)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_to_axis_angle(self) -> (Vec3, f32) {
        let mut q = self.unchecked_normalize();

        if q.w < 0.0 {
            q = q.unchecked_neg();
        }

        let sin_half = q.x.hypot(q.y).hypot(q.z);

        if sin_half == 0.0 {
            return (Vec3::unchecked_new(1.0, 0.0, 0.0), 0.0);
        }

        let axis = Vec3::unchecked_new(q.x / sin_half, q.y / sin_half, q.z / sin_half);

        let angle = 2.0 * sin_half.atan2(q.w).to_degrees();

        (axis, angle)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_slerp(self, end: Self, t: f32) -> Self {
        let start = self.unchecked_normalize();
        let mut end = end.unchecked_normalize();
        let mut dot = start.unchecked_dot(end);

        if t == 0.0 {
            return start;
        }

        if t == 1.0 {
            return end;
        }

        if dot < 0.0 {
            end = end.unchecked_neg();
            dot = -dot;
        }

        dot = dot.clamp(0.0, 1.0);

        const DOT_THRESHOLD: f32 = 0.9995;

        if dot > DOT_THRESHOLD {
            return start
                .unchecked_mul_scalar(1.0 - t)
                .unchecked_add(end.unchecked_mul_scalar(t))
                .unchecked_normalize();
        }

        let theta = dot.acos();
        let sin_theta = theta.sin();

        if !sin_theta.is_finite() || sin_theta == 0.0 {
            return start
                .unchecked_mul_scalar(1.0 - t)
                .unchecked_add(end.unchecked_mul_scalar(t))
                .unchecked_normalize();
        }

        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;

        start
            .unchecked_mul_scalar(a)
            .unchecked_add(end.unchecked_mul_scalar(b))
            .unchecked_normalize()
    }
}
