# Mat4

A 4×4 matrix for spatial transformations: translation, rotation, scale,
composition, projection, and view matrices.

## Example

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{Mat4, Quat, Vec3};

let model = Mat4::from_scale_rotation_translation(
    Vec3::splat(1.0)?,
    Quat::from_rotation_y(45.0)?,
    Vec3::new(0.0, 5.0, 0.0)?,
)?;

let view = Mat4::look_at_rh(
    Vec3::new(0.0, 2.0, 10.0)?,   // eye
    Vec3::new(0.0, 1.0, 0.0)?,    // center
    Vec3::UP,                     // up
)?;

let projection = Mat4::perspective_rh(60.0, 16.0 / 9.0, 0.1, 100.0)?;
let mvp = ((projection * view)? * model)?;

let point = model.transform_point3(Vec3::ZERO)?;
# Ok(())
# }
```

## Layout: column-major

`Mat4` stores four column vectors (`x_axis` … `w_axis`):

- [`Mat4::to_array`] — 16 floats, column-major (OpenGL/WebGPU order)
  (`const fn`)
- [`Mat4::to_cols_array_2d`] — `[[f32; 4]; 4]` indexed `[column][row]`
  (`const fn`)
- [`Mat4::from_array`] — reads the same column-major order (`const fn`)

The translation of an affine matrix lives in the last column (`w_axis`).

## Constructors

All constructors return `Result` and reject non-finite inputs:

| Method | Description |
|---|---|
| `new` / `from_cols` / `from_array` | From columns or a flat array (`const fn`) |
| `from_translation(v)` | Translation matrix (`const fn`) |
| `try_from_scale(v)` | Scale matrix (`const fn`) |
| `from_quat(q)` | Rotation matrix; normalizes `q` first |
| `from_scale_rotation_translation(s, r, t)` | SRT composition (scale → rotate → translate) |
| `look_at_rh(eye, center, up)` | Right-handed view matrix; degenerate basis → `InvalidLookAtParameters` |
| `perspective_rh(fov_y_degrees, aspect, near, far)` | Right-handed projection, depth `[-1, 1]`; bad parameters → `InvalidPerspectiveParameters` |
| `orthographic_rh(l, r, b, t, near, far)` | Right-handed orthographic, depth `[-1, 1]`; degenerate ranges → `InvalidOrthographicParameters` |

[`Mat4::IDENTITY`] and `Default` give the identity matrix.

## Applying transforms

- `transform_point3(v)` — treats `v` as a point (`w = 1`) and divides by
  the resulting homogeneous `w`; `w == 0` → `ZeroHomogeneousCoordinate`.
- `transform_vector3(v)` — treats `v` as a direction (`w = 0`);
  translation is ignored.
- Operators `m * v` ([`Vec4`]) and `m1 * m2` return `Result`; `(a * b)`
  applies `b` first, then `a`.
- `checked_mul_assign(other)` — in-place multiplication with an explicit
  error.

## Inversion and determinant

- `inverse()` — Gauss-Jordan with partial pivoting in `f64`; a zero pivot
  → `SingularMatrix` (`const fn`)
- `determinant()` — `f64` internally, returned as `f32` (`const fn`)
- `transpose()` — swaps rows and columns (`const fn`)

All three work in const contexts.

## Errors

[`Mat4Error`]: `NonFiniteInput`, `SingularMatrix`,
`InvalidPerspectiveParameters`, `InvalidOrthographicParameters`,
`InvalidLookAtParameters`, `InvalidQuaternion`,
`ZeroHomogeneousCoordinate`, `InvalidResult`. See `docs/errors.md`.

## See also

Repository guides: `docs/matrices.md`.
