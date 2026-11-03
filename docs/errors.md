# Error handling

Plam's central idea: **mathematical failures are explicit**. Instead of
silently producing `NaN`, infinity, or a degenerate result, fallible
operations return a `Result` you must handle.

## What can fail

| Situation | Example API | Error variant |
|---|---|---|
| Non-finite input (`NaN`, `±inf`) | `Vec3::new(f32::NAN, 0.0, 0.0)` | `NonFiniteInput` |
| Division by zero | `v / Vec3::ZERO`, `v / 0.0` | `DivisionByZero` |
| Normalizing a zero-length value | `Vec3::ZERO.normalize()`, `Quat::ZERO.normalize()` | `ZeroLength` |
| Singular matrix inversion | `m.inverse()` | `SingularMatrix` |
| Operation overflowed to non-finite | any arithmetic | `InvalidResult` |
| Bad projection parameters | `perspective_rh(200.0, …)` | `InvalidPerspectiveParameters` |
| Degenerate orthographic ranges | `orthographic_rh(0.0, 0.0, …)` | `InvalidOrthographicParameters` |
| Degenerate look-at basis | `look_at_rh(eye, eye, up)` | `InvalidLookAtParameters` |
| Zero homogeneous `w` | `m.transform_point3(…)` | `ZeroHomogeneousCoordinate` |
| Zero rotation axis | `Quat::from_axis_angle(Vec3::ZERO, 90.0)` | `InvalidAxis` |
| Input out of range (input types) | `MouseDelta::new(2e6, 0.0)` | `OutOfRange` |

Each type has its own error enum: `Vec2Error`, `Vec3Error`, `Vec4Error`,
`QuatError`, `Mat4Error`, `MouseDeltaError`, `MouseScrollDeltaError`.
All implement [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
via `thiserror`, are `Copy + PartialEq + Eq + Hash`, and (with the `serde`
feature) are serializable.

## What does *not* fail

- **Constructors of valid-by-default values:** `Vec3::ZERO`, `Quat::IDENTITY`,
  `Mat4::IDENTITY` are constants — using them is always fine.
- **Queries:** `is_finite()`, `is_zero()`, `to_array()`, accessors.
- **WindowSize / SurfaceSize:** out-of-range sizes are *clamped*, not
  rejected (see [Window and input types](window-and-input.md)).

## Patterns

### Propagate with `?`

```rust
use plam::{Vec3, Vec3Error};

fn direction_length(d: Vec3) -> Result<f32, Vec3Error> {
    Ok(d.normalize()?.length()?)
}
```

### Handle at the edge

```rust
use plam::Vec3;

fn normalize_or_default(v: Vec3) -> Vec3 {
    // Only pick a fallback when identity/zero is a *valid* answer for
    // your logic — otherwise propagate the error instead.
    v.normalize().unwrap_or(Vec3::ZERO)
}
```

### Match on the error

```rust
use plam::{Vec3, Vec3Error};

fn safe_normalize(v: Vec3) -> Result<Vec3, String> {
    match v.normalize() {
        Ok(unit) => Ok(unit),
        Err(Vec3Error::ZeroLength) => Err("zero-length direction".into()),
        Err(other) => Err(other.to_string()),
    }
}
```

## Design guidance

The README of the crate says it best:

> Avoid silently replacing errors with arbitrary fallback values. A zero
> vector, identity quaternion, or identity matrix should only be used when
> it is a valid result for the intended operation.

Rules of thumb:

1. **Propagate** errors you cannot interpret (`InvalidResult`,
   `NonFiniteInput` — these usually indicate a bug or corrupted data).
2. **Handle** domain errors where your domain has a sensible answer
   (`ZeroLength` → "no direction chosen yet").
3. **Never** turn an error into a plausible-looking number just to satisfy
   the type system.

## Validity model

- **Valid by construction:** vectors, quaternions, and matrices can only
  hold finite components. You cannot create an invalid instance through
  the public API (fields are private).
- **Not enforced at construction:** unit length for `Quat`, invertibility
  for `Mat4`. These are checked when it matters — `normalize()`,
  `rotate_vec3()`, `from_quat()`, `inverse()`.
- **Deserialization is validated:** the `serde` impls route through the
  same constructors, so external data cannot bypass the checks.
