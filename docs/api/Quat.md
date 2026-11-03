# Quat

A quaternion representing a rotation in 3D space. Quaternions compose
rotations without gimbal lock and are the preferred way to orient objects.

## Example

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{Quat, Vec3};

// Angles are degrees
let yaw = Quat::from_rotation_y(90.0)?;
let pitch = Quat::from_rotation_x(-30.0)?;

let combined = (yaw * pitch)?;                       // apply pitch, then yaw
let forward = combined.rotate_vec3(Vec3::FORWARD)?;

let halfway = Quat::IDENTITY.slerp(yaw, 0.5)?;       // 45° around Y
let (axis, degrees) = yaw.to_axis_angle()?;
# Ok(())
# }
```

## Key facts

- **Angles are degrees** in every constructor and in [`Quat::to_axis_angle`].
- **Not required to be unit length** at construction; operations that need
  a unit quaternion (`rotate_vec3`, `Mat4::from_quat`) normalize internally.
- [`Quat::IDENTITY`] is "no rotation"; [`Quat::ZERO`] is *not* a valid
  rotation and is rejected by `normalize()`, `inverse()`, `rotate_vec3()`.
- Components are private — read them with `x()`, `y()`, `z()`, `w()`,
  `xyz()`, or `to_array()` (all `const fn`).

## Constructors

| Method | Description |
|---|---|
| `new(x, y, z, w)` | Finite check only, no unit-length requirement (`const fn`) |
| `from_array([f32; 4])` / `splat(value)` | Construction helpers (`const fn`) |
| `from_axis_angle(axis, degrees)` | Axis does not need to be normalized; zero axis → `InvalidAxis` |
| `from_rotation_x/y/z(degrees)` | Shortcuts for the +X / +Y / +Z axes |
| `from_euler(x, y, z)` | Euler angles in degrees, wrapped with `rem_euclid(360)` |
| `from_scaled_axis(v)` | Magnitude of `v` is an angle in **degrees** (differs from libraries that use radians) |

Rotations follow the right-hand rule: `from_rotation_y(90°)` maps
`(1,0,0)` to `(0,0,-1)`; `from_rotation_z(90°)` maps `(1,0,0)` to `(0,1,0)`.

## Working with rotations

| Method | Description |
|---|---|
| `rotate_vec3(v)` | Rotate a vector; normalizes `self` first |
| `to_axis_angle()` | Canonical pair: angle in `[0, 180]`, `w ≥ 0` |
| `slerp(end, t)` | Spherical interpolation, shortest arc, unit result |
| `conjugate()` | Inverse rotation for unit quaternions (`const fn`) |
| `inverse()` | True inverse for any non-zero quaternion, computed in `f64` (`const fn`) |
| `normalize()` / `length()` / `length_squared()` | Magnitude operations |
| `dot(other)` | Quaternion dot product (`const fn`) |
| `xyz()` | Vector part as [`Vec3`] (`const fn`) |

`slerp` normalizes both endpoints, takes the shortest path (negates `end`
when `dot < 0`), and falls back to `lerp + normalize` for near-parallel
inputs. `t` outside `[0, 1]` extrapolates.

## Operators

`q1 * q2` composes rotations (apply `q2` first, then `q1`). `+`, `-`,
`*` (quaternion or scalar), `/`, and unary `-` all return
`Result<Quat, QuatError>` — chain as `((q1 * q2)? * q3)?`.

## Errors

[`QuatError`]: `NonFiniteInput`, `InvalidResult`, `DivisionByZero`,
`ZeroLength`, `InvalidAxis`. See `docs/errors.md`.

## See also

Repository guides: `docs/quaternions.md`.
