# Matrices

`Mat4` is a 4×4 matrix used for spatial transformations: translation,
rotation, scale, composition, and perspective projection.

## Layout: column-major

`Mat4` stores four column vectors:

```rust
use plam::Mat4;

let m = Mat4::IDENTITY;
let [x_axis, y_axis, z_axis, w_axis] = m.to_cols_array_2d();
```

- `to_array()` returns 16 floats in **column-major** order
  (`m11, m12, m13, m14, m21, …`), matching OpenGL/WebGPU conventions.
- `to_cols_array_2d()` returns `[[f32; 4]; 4]` indexed as `[column][row]`.
- `from_array([f32; 16])` reads the same column-major order.

The translation of an affine matrix lives in the last column (`w_axis`).

## Construction

All constructors return `Result` and reject non-finite inputs:

```rust
use plam::{Mat4, Quat, Vec3};

let translation = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)?)?;
let scale = Mat4::try_from_scale(Vec3::new(2.0, 2.0, 2.0)?)?;
let rotation = Mat4::from_quat(Quat::from_rotation_y(90.0)?)?;

// SRT composition: scale first, then rotation, then translation
let model = Mat4::from_scale_rotation_translation(
    Vec3::splat(1.0)?,
    Quat::from_rotation_y(45.0)?,
    Vec3::new(0.0, 5.0, 0.0)?,
)?;

// From raw columns
let custom = Mat4::from_cols(
    plam::Vec4::new(1.0, 0.0, 0.0, 0.0)?,
    plam::Vec4::new(0.0, 1.0, 0.0, 0.0)?,
    plam::Vec4::new(0.0, 0.0, 1.0, 0.0)?,
    plam::Vec4::new(0.0, 0.0, 0.0, 1.0)?,
)?;
```

> `Mat4::IDENTITY` and `Default` give the identity matrix.

## Applying transforms

```rust
use plam::{Mat4, Vec3};

let model = Mat4::from_translation(Vec3::new(10.0, 0.0, 0.0)?)?;

let point = model.transform_point3(Vec3::new(1.0, 0.0, 0.0)?)?;   // (11, 0, 0)
let direction = model.transform_vector3(Vec3::new(1.0, 0.0, 0.0)?)?; // (1, 0, 0)
```

- `transform_point3` treats the input as a point (`w = 1`), divides by the
  resulting homogeneous `w`, and fails with `ZeroHomogeneousCoordinate`
  when `w == 0`.
- `transform_vector3` treats the input as a direction (`w = 0`), so
  translation is ignored.

The `Mul` operator applies to both vectors and matrices:

```rust
use plam::{Mat4, Vec4};

let m = Mat4::IDENTITY;
let v = Vec4::new(1.0, 2.0, 3.0, 1.0)?;
let mv = (m * v)?;

// Matrix product: (a * b) applies b first, then a
let ab = (m * Mat4::IDENTITY)?;
```

`checked_mul_assign` is available when you want in-place multiplication
with an explicit error:

```rust
use plam::Mat4;

let mut m = Mat4::IDENTITY;
m.checked_mul_assign(Mat4::IDENTITY)?;
```

## Inversion and determinants

```rust
use plam::{Mat4, Vec3};

let m = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)?)?;
let inverse = m.inverse()?;             // Gauss-Jordan with partial pivoting (f64)
let det = m.determinant()?;             // f64 internally, returned as f32

// Round-trip: inverse * m ≈ identity
let round_trip = (inverse * m)?;
```

- `inverse()` fails with `Mat4Error::SingularMatrix` when a pivot is zero.
- Both routines compute in `f64` internally and convert back to `f32`
  with range checks (`InvalidResult` on overflow).
- `transpose()` swaps rows and columns.
- `inverse()`, `determinant()`, and `transpose()` are `const fn` and work
  in const contexts.

## Look-at views

`look_at_rh` builds a right-handed **view matrix** that places an observer
at `eye`, looking toward `center`, with `up` as the reference up direction:

```rust
use plam::{Mat4, Vec3};

let view = Mat4::look_at_rh(
    Vec3::new(0.0, 2.0, 10.0)?,   // eye
    Vec3::new(0.0, 1.0, 0.0)?,    // center (what the camera looks at)
    Vec3::new(0.0, 1.0, 0.0)?,    // up
)?;

// The eye maps to the origin of view space
let at_eye = view.transform_point3(Vec3::new(0.0, 2.0, 10.0)?)?;
// The center lies straight ahead, along -Z
let at_center = view.transform_point3(Vec3::new(0.0, 1.0, 0.0)?)?;
```

Degenerate configurations fail with `Mat4Error::InvalidLookAtParameters`:

- `eye == center` (no view direction),
- `up` parallel to the view direction (the basis cannot be built).

The `up` vector does not need to be normalized.

## Perspective projection

```rust
use plam::Mat4;

// Right-handed, clip depth in [-1, 1] (OpenGL style)
let proj = Mat4::perspective_rh(
    60.0,      // vertical field of view, degrees
    16.0 / 9.0, // aspect ratio (width / height)
    0.1,       // near plane (> 0)
    100.0,     // far plane (> near)
)?;
```

Invalid parameters (non-finite, FOV outside `(0, 180)`, aspect ≤ 0,
near ≤ 0, far ≤ near) fail with `Mat4Error::InvalidPerspectiveParameters`.

A typical frame setup:

```rust
use plam::{Mat4, Quat, Vec3};

fn frame(aspect: f32) -> Result<Mat4, Box<dyn std::error::Error>> {
    let projection = Mat4::perspective_rh(60.0, aspect, 0.1, 100.0)?;

    // View matrix: inverse of the camera transform
    let camera = Mat4::from_scale_rotation_translation(
        Vec3::splat(1.0)?,
        Quat::from_rotation_y(0.0)?,
        Vec3::new(0.0, 2.0, 10.0)?,
    )?;
    let view = camera.inverse()?;

    let model = Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)?)?;

    Ok(((projection * view)? * model)?)
}
```

## Orthographic projection

`orthographic_rh` builds a right-handed orthographic projection with clip
depth in `[-1, 1]` (OpenGL style) — the standard choice for 2D games, UI,
and CAD-style views where parallel lines must stay parallel:

```rust
use plam::Mat4;

let proj = Mat4::orthographic_rh(
    -8.0, 8.0,    // left, right
    -4.5, 4.5,    // bottom, top
    0.1, 100.0,   // near, far
)?;
```

Parameters:

- All six values must be finite.
- `left != right`, `bottom != top`, and `near != far`; otherwise the call
  fails with `Mat4Error::InvalidOrthographicParameters`.
- Unlike `perspective_rh`, negative near/far values are not rejected —
  only degenerate ranges are.

A view-space point maps to normalized device coordinates as:

- `x → [-1, 1]` across `[left, right]`
- `y → [-1, 1]` across `[bottom, top]`
- `z → [-1, 1]` across `[-far, -near]` (near plane → −1, far plane → +1)

## Serialization

With `serde`, `Mat4` serializes as a struct of four columns (`x_axis` …
`w_axis`). Non-finite values are rejected on serialize; deserialization
validates each column.

## Errors

| Error | When |
|---|---|
| `NonFiniteInput` | Any matrix or operand component is `NaN`/`±inf` |
| `SingularMatrix` | `inverse()` on a matrix with a zero pivot |
| `InvalidPerspectiveParameters` | Bad FOV/aspect/near/far |
| `InvalidOrthographicParameters` | Degenerate left/right, bottom/top, or near/far ranges |
| `InvalidLookAtParameters` | `eye == center`, or `up` parallel to the view direction |
| `InvalidQuaternion` | `from_quat` could not normalize the rotation |
| `ZeroHomogeneousCoordinate` | `transform_point3` divided by `w == 0` |
| `InvalidResult` | Overflow or non-finite result |

See [Error handling](errors.md) for general patterns.
