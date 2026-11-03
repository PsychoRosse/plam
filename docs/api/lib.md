# Plam

**Plam** is a safe mathematics library for Rust, designed for game development,
3D graphics, and geometric computations.

It provides essential mathematical primitives with explicit error handling,
input validation, and predictable behavior.

## Features

- 2D and 3D vectors ([`Vec2`], [`Vec3`], [`Vec4`])
- Quaternions ([`Quat`]) for representing rotations
- 4×4 matrices ([`Mat4`]) for spatial transformations
- Vector and matrix arithmetic
- Rotation, translation, scaling, perspective and orthographic projection
- Look-at view matrices
- Linear and spherical interpolation ([`Vec3::lerp`], [`Quat::slerp`])
- Many methods are `const fn` and work in const contexts
- Serialization and deserialization (`serde` feature, on by default)
- Explicit error handling for fallible operations

## Quick start

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{Mat4, Quat, Vec3};

// Vectors: construction validates that components are finite
let position = Vec3::new(1.0, 2.0, 3.0)?;

// Rotations use degrees
let rotation = Quat::from_rotation_y(90.0)?;

// Matrices compose spatial transforms
let model = Mat4::from_scale_rotation_translation(
    Vec3::splat(1.0)?,
    rotation,
    position,
)?;

let point = model.transform_point3(Vec3::ZERO)?;
println!("{point:?}");
# Ok(())
# }
```

## Design principles

- **Safety:** inputs are validated and invalid numeric states (`NaN`,
  infinity, zero-length normalization, singular inversion) are rejected.
- **Explicit errors:** fallible operations return `Result`. Operators such
  as `+`, `-`, `*`, `/` return `Result` as well, so chain them as
  `((a + b)? + c)?`.
- **Predictability:** no silently substituted fallback values — a zero
  vector or identity quaternion is only produced when it is a valid result.
- **Validity by construction:** vector, quaternion, and matrix fields are
  private; every public constructor enforces finiteness.

## Conventions

- **Angles are degrees** everywhere ([`Quat::from_euler`],
  [`Quat::from_axis_angle`], [`Mat4::perspective_rh`], …).
- **Matrices are column-major** ([`Mat4::to_array`]).
- **Components are read** through the `x()`, `y()`, `z()`, `w()` accessors
  or `to_array()`.
- **`const fn`:** construction, queries, and pure arithmetic are `const`
  where Rust allows. Methods that need `sqrt` or trigonometry
  ([`Vec3::length`], [`Vec3::normalize`], [`Quat::slerp`],
  [`Mat4::perspective_rh`]) are runtime-only.

## Errors

Every fallible area has its own error type: [`Vec2Error`], [`Vec3Error`],
[`Vec4Error`], [`QuatError`], [`Mat4Error`], [`MouseDeltaError`],
[`MouseScrollDeltaError`]. All implement [`std::error::Error`] via
`thiserror`, are `Copy + PartialEq + Eq + Hash`, and are serializable with
the `serde` feature.

## Features (Cargo)

| Feature | Default | Description |
|---|---|---|
| `serde` | yes | `Serialize` / `Deserialize` for all public types, with validation |
| `clama` | yes | Derives `clama::Event` for the window and input types |

## Repository guides

The repository contains extended guides in the `docs/` folder: getting
started, vectors, quaternions, matrices, error handling, and window/input
types. This API reference is generated from the markdown files in
`docs/api/`.

[`Vec2`]: crate::Vec2
[`Vec3`]: crate::Vec3
[`Vec4`]: crate::Vec4
[`Quat`]: crate::Quat
[`Mat4`]: crate::Mat4
[`Vec3::lerp`]: crate::Vec3::lerp
[`Vec3::length`]: crate::Vec3::length
[`Vec3::normalize`]: crate::Vec3::normalize
[`Quat::slerp`]: crate::Quat::slerp
[`Quat::from_euler`]: crate::Quat::from_euler
[`Quat::from_axis_angle`]: crate::Quat::from_axis_angle
[`Mat4::perspective_rh`]: crate::Mat4::perspective_rh
[`Mat4::to_array`]: crate::Mat4::to_array
[`Vec2Error`]: crate::Vec2Error
[`Vec3Error`]: crate::Vec3Error
[`Vec4Error`]: crate::Vec4Error
[`QuatError`]: crate::QuatError
[`Mat4Error`]: crate::Mat4Error
[`MouseDeltaError`]: crate::MouseDeltaError
[`MouseScrollDeltaError`]: crate::MouseScrollDeltaError
