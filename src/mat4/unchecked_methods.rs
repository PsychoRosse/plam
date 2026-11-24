use crate::{Vec3, Vec4};

use super::Mat4;

impl Mat4 {
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
        let result = self.unchecked_mul_vec4(Vec4::unchecked_new(
            point.x,
            point.y,
            point.z,
            1.0,
        ));

        let x = result.x as f64 / result.w as f64;
        let y = result.y as f64 / result.w as f64;
        let z = result.z as f64 / result.w as f64;

        Vec3::unchecked_new(x as f32, y as f32, z as f32)
    }

    #[inline]
    #[must_use]
    pub fn unchecked_transform_vector3(self, vector: Vec3) -> Vec3 {
        let result = self.unchecked_mul_vec4(Vec4::unchecked_new(
            vector.x,
            vector.y,
            vector.z,
            0.0,
        ));

        Vec3::unchecked_new(result.x, result.y, result.z)
    }
}
