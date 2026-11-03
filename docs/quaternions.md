# Quaternions

`Quat` represents a rotation in 3D space. Quaternions compose rotations
without gimbal lock and are the preferred way to orient objects in Plam.

```rust
use plam::Quat;

let q = Quat::from_rotation_y(90.0)?;   // 90 degrees
```

## Angles are degrees

Every angle-taking and angle-returning API in Plam uses **degrees**:

- `Quat::from_euler`, `from_axis_angle`, `from_rotation_x/y/z`
- `Quat::to_axis_angle` (returned angle)
- `Quat::from_scaled_axis` (magnitude of the scaled axis, see below)
- `Mat4::perspective_rh` (field of view)

`std` trigonometry uses radians, so convert at the boundary with
`.to_radians()` if your data comes from elsewhere.

## Constants and construction

```rust
use plam::Quat;

let identity = Quat::IDENTITY;   // no rotation
let zero = Quat::ZERO;           // (0,0,0,0) — NOT a valid rotation
```

`Quat::ZERO` exists for completeness but is rejected by `normalize()`,
`inverse()`, `rotate_vec3()`, and `Mat4::from_quat()`. Do not use it as a
"no rotation" value — that is `IDENTITY`.

Building from components is fallible (finite check only; unit length is
**not** required at construction):

```rust
use plam::Quat;

let q = Quat::new(0.0, 0.0, 0.0, 1.0)?;
let from_array = Quat::from_array([0.0, 0.0, 0.0, 1.0])?;
```

## Creating rotations

### Axis-angle

```rust
use plam::{Quat, Vec3};

// 45 degrees around the up axis
let q = Quat::from_axis_angle(Vec3::UP, 45.0)?;

// The same, via shortcuts (axes are +X, +Y, +Z respectively)
let qx = Quat::from_rotation_x(45.0)?;
let qy = Quat::from_rotation_y(45.0)?;
let qz = Quat::from_rotation_z(45.0)?;
```

A zero axis is rejected with `QuatError::InvalidAxis`. The axis does not
need to be normalized — Plam normalizes it internally.

Rotations follow the right-hand rule: `from_rotation_y(90°)` maps
`(1,0,0)` to `(0,0,-1)`; `from_rotation_z(90°)` maps `(1,0,0)` to `(0,1,0)`.

### Euler angles

```rust
use plam::Quat;

// X, Y, Z applied in order; each in degrees, any magnitude (wrapped mod 360)
let q = Quat::from_euler(30.0, 45.0, 0.0)?;
```

Angles are wrapped with `rem_euclid(360.0)`, so any finite degree value is
accepted.

### Scaled axis

```rust
use plam::{Quat, Vec3};

let q = Quat::from_scaled_axis(Vec3::new(0.0, 180.0, 0.0)?)?;
```

The magnitude of the vector is interpreted as a rotation angle in
**degrees** around its direction (a zero vector yields `IDENTITY`). Note
that this convention differs from libraries that treat the scaled-axis
magnitude as radians — convert accordingly when mixing sources.

## Working with rotations

### Applying a rotation

```rust
use plam::{Quat, Vec3};

let q = Quat::from_rotation_z(90.0)?;
let rotated = q.rotate_vec3(Vec3::new(1.0, 0.0, 0.0)?)?;
// rotated ≈ (0, 1, 0)
```

`rotate_vec3` normalizes the quaternion first, so non-unit quaternions
still rotate correctly.

### Extracting an axis-angle

```rust
use plam::Quat;

let q = Quat::from_rotation_y(90.0)?;
let (axis, degrees) = q.to_axis_angle()?;
// axis ≈ (0, 1, 0), degrees ≈ 90
```

The returned angle is in `[0, 180]` and the quaternion's `w` is kept
non-negative, so the pair is canonical.

### Interpolation

`slerp(end, t)` performs a spherical linear interpolation between two
rotations — constant angular velocity along the shortest arc:

```rust
use plam::Quat;

let start = Quat::IDENTITY;
let end = Quat::from_rotation_y(90.0)?;

let half = start.slerp(end, 0.5)?;    // 45° around Y
let done = start.slerp(end, 1.0)?;    // ≈ end
```

Behaviour details:

- Both quaternions are normalized first; the result is always unit length.
- The **shortest path** is taken: if `end` points into the opposite
  hemisphere (`dot < 0`), it is negated internally.
- Near-parallel inputs fall back to `lerp` + `normalize`, which is more
  numerically stable than the spherical formula there.
- `t` outside `[0, 1]` extrapolates; `t` must be finite.
- `Quat::ZERO` as either endpoint fails with `ZeroLength`.

For frame-to-frame rotation smoothing, prefer `slerp` over component-wise
`lerp`: it preserves unit length without extra renormalization drift.

### Algebra

| Method / operator | Meaning |
|---|---|
| `q1 * q2` | Composition: apply `q2` first, then `q1` |
| `q.conjugate()?` | Inverse rotation for unit quaternions |
| `q.inverse()?` | True inverse for any non-zero quaternion (computed in `f64`) |
| `q.normalize()?` | Scale to unit length |
| `q.dot(other)?` | Quaternion dot product |
| `q.slerp(end, t)?` | Spherical interpolation (shortest arc, unit result) |
| `q.length()?` / `q.length_squared()?` | Magnitude |
| `q.xyz()?` | Vector part as `Vec3` |
| `q + r`, `q - r`, `q * k`, `q / k`, `-q` | Component-wise, all fallible |

Like vectors, the operators return `Result`, so chain as `((q1 * q2)? * q3)?`.

`conjugate()`, `inverse()`, `dot()`, `length_squared()`, and `xyz()` are
`const fn`; methods that need `sqrt` or trigonometry (`length()`,
`normalize()`, `slerp()`, rotation constructors) are runtime-only.

Composition example:

```rust
use plam::Quat;

let yaw = Quat::from_rotation_y(90.0)?;
let pitch = Quat::from_rotation_x(-30.0)?;

// yaw after pitch: apply pitch first, then yaw
let combined = (yaw * pitch)?;

let v = combined.rotate_vec3(plam::Vec3::FORWARD)?;
```

## Errors

| Error | When |
|---|---|
| `NonFiniteInput` | A component or argument is `NaN`/`±inf` |
| `ZeroLength` | Normalizing or inverting `(0,0,0,0)` |
| `InvalidAxis` | `from_axis_angle` received a zero axis |
| `DivisionByZero` | `q / 0.0` |
| `InvalidResult` | The computation overflowed or produced a non-finite value |

See [Error handling](errors.md) for general patterns.

## Serialization

With `serde`, `Quat` serializes as a struct with `x`, `y`, `z`, `w` fields.
Serialization rejects non-finite components; deserialization validates via
`Quat::new` but does not require unit length (normalize after loading if
your pipeline needs it).
