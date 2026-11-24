use crate::{Quat, Vec3, Vec4};

use super::Mat4;

impl Mat4 {
    #[inline]
    #[must_use]
    pub fn unchecked_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let forward = center.unchecked_sub(eye);
        let forward_length = forward.unchecked_length();
        let forward = forward.unchecked_div_scalar(forward_length);

        let side = forward.unchecked_cross(up);
        let side_length = side.unchecked_length();
        let side = side.unchecked_div_scalar(side_length);
        let up_corrected = side.unchecked_cross(forward);

        let side_dot = side.unchecked_dot(eye);
        let up_dot = up_corrected.unchecked_dot(eye);
        let forward_dot = forward.unchecked_dot(eye);

        Self::unchecked_new(
            Vec4::unchecked_new(side.x, up_corrected.x, -forward.x, 0.0),
            Vec4::unchecked_new(side.y, up_corrected.y, -forward.y, 0.0),
            Vec4::unchecked_new(side.z, up_corrected.z, -forward.z, 0.0),
            Vec4::unchecked_new(-side_dot, -up_dot, forward_dot, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self::unchecked_from_orthographic_parts(left, right, bottom, top, near, far)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_translation(translation: Vec3) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(1.0, 0.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 1.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 0.0, 1.0, 0.0),
            Vec4::unchecked_new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_scale(scale: Vec3) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(scale.x, 0.0, 0.0, 0.0),
            Vec4::unchecked_new(0.0, scale.y, 0.0, 0.0),
            Vec4::unchecked_new(0.0, 0.0, scale.z, 0.0),
            Vec4::unchecked_new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_quat(rotation: Quat) -> Self {
        let q = rotation.unchecked_normalize();

        let [x, y, z, w] = q.to_array();

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;

        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;

        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        Self::unchecked_new(
            Vec4::unchecked_new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0),
            Vec4::unchecked_new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0),
            Vec4::unchecked_new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0),
            Vec4::unchecked_new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_from_scale_rotation_translation(
        scale: Vec3,
        rotation: Quat,
        translation: Vec3,
    ) -> Self {
        let rotation = Self::unchecked_from_quat(rotation);

        Self::unchecked_new(
            Vec4::unchecked_new(
                rotation.x_axis.x * scale.x,
                rotation.x_axis.y * scale.x,
                rotation.x_axis.z * scale.x,
                0.0,
            ),
            Vec4::unchecked_new(
                rotation.y_axis.x * scale.y,
                rotation.y_axis.y * scale.y,
                rotation.y_axis.z * scale.y,
                0.0,
            ),
            Vec4::unchecked_new(
                rotation.z_axis.x * scale.z,
                rotation.z_axis.y * scale.z,
                rotation.z_axis.z * scale.z,
                0.0,
            ),
            Vec4::unchecked_new(translation.x, translation.y, translation.z, 1.0),
        )
    }

    #[inline]
    #[must_use]
    pub fn unchecked_perspective_rh(
        fov_y_degrees: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self::unchecked_from_perspective_parts(fov_y_degrees, aspect_ratio, near, far)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_cols(
        x_axis: Vec4,
        y_axis: Vec4,
        z_axis: Vec4,
        w_axis: Vec4,
    ) -> Self {
        Self::unchecked_new(x_axis, y_axis, z_axis, w_axis)
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_from_array(array: [f32; 16]) -> Self {
        Self::unchecked_new(
            Vec4::unchecked_new(array[0], array[1], array[2], array[3]),
            Vec4::unchecked_new(array[4], array[5], array[6], array[7]),
            Vec4::unchecked_new(array[8], array[9], array[10], array[11]),
            Vec4::unchecked_new(array[12], array[13], array[14], array[15]),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_transpose(self) -> Self {
        let a = self.to_cols_array_2d();

        Self::unchecked_new(
            Vec4::unchecked_new(a[0][0], a[1][0], a[2][0], a[3][0]),
            Vec4::unchecked_new(a[0][1], a[1][1], a[2][1], a[3][1]),
            Vec4::unchecked_new(a[0][2], a[1][2], a[2][2], a[3][2]),
            Vec4::unchecked_new(a[0][3], a[1][3], a[2][3], a[3][3]),
        )
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_determinant(self) -> f32 {
        self.unchecked_determinant_f64() as f32
    }

    #[inline]
    pub(crate) const fn unchecked_determinant_f64(self) -> f64 {
        let array = self.to_array();
        let mut a = [[0.0_f64; 4]; 4];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                a[row][col] = array[col * 4 + row] as f64;
                col += 1;
            }

            row += 1;
        }

        let mut determinant = 1.0_f64;
        let mut col = 0;

        while col < 4 {
            let mut pivot = col;
            let mut scan = col + 1;

            while scan < 4 {
                if a[scan][col].abs() > a[pivot][col].abs() {
                    pivot = scan;
                }

                scan += 1;
            }

            if a[pivot][col] == 0.0 {
                return 0.0;
            }

            if pivot != col {
                a.swap(pivot, col);
                determinant = -determinant;
            }

            let pivot_value = a[col][col];
            determinant *= pivot_value;

            let mut target = col + 1;

            while target < 4 {
                let factor = a[target][col] / pivot_value;
                let mut k = col + 1;

                while k < 4 {
                    a[target][k] -= factor * a[col][k];
                    k += 1;
                }

                target += 1;
            }

            col += 1;
        }

        determinant
    }

    #[inline]
    #[must_use]
    pub const fn unchecked_inverse(self) -> Self {
        let array = self.to_array();
        let mut a = [[0.0_f64; 8]; 4];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                a[row][col] = array[col * 4 + row] as f64;
                col += 1;
            }

            a[row][row + 4] = 1.0;
            row += 1;
        }

        let mut col = 0;

        while col < 4 {
            let mut pivot = col;
            let mut scan = col + 1;

            while scan < 4 {
                if a[scan][col].abs() > a[pivot][col].abs() {
                    pivot = scan;
                }

                scan += 1;
            }

            a.swap(pivot, col);

            let divisor = a[col][col];
            let mut k = 0;

            while k < 8 {
                a[col][k] /= divisor;
                k += 1;
            }

            let mut elim = 0;

            while elim < 4 {
                if elim != col {
                    let factor = a[elim][col];
                    let mut idx = 0;

                    while idx < 8 {
                        a[elim][idx] -= factor * a[col][idx];
                        idx += 1;
                    }
                }

                elim += 1;
            }

            col += 1;
        }

        let mut inverse = [0.0_f64; 16];
        let mut row = 0;

        while row < 4 {
            let mut col = 0;

            while col < 4 {
                inverse[col * 4 + row] = a[row][col + 4];
                col += 1;
            }

            row += 1;
        }

        Self::unchecked_from_f64_array(inverse)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_transform_point3(self, point: Vec3) -> Vec3 {
        let result = self.unchecked_mul_vec4(Vec4::unchecked_new(point.x, point.y, point.z, 1.0));

        let x = result.x as f64 / result.w as f64;
        let y = result.y as f64 / result.w as f64;
        let z = result.z as f64 / result.w as f64;

        Vec3::unchecked_new(x as f32, y as f32, z as f32)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_transform_vector3(self, vector: Vec3) -> Vec3 {
        let result =
            self.unchecked_mul_vec4(Vec4::unchecked_new(vector.x, vector.y, vector.z, 0.0));

        Vec3::unchecked_new(result.x, result.y, result.z)
    }
}
