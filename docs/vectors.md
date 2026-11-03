# Vectors

Plam provides three vector types:

| Type | Components | Typical use |
|---|---|---|
| `Vec2` | `x`, `y` | 2D positions, directions, UV coordinates |
| `Vec3` | `x`, `y`, `z` | 3D positions, directions, velocities |
| `Vec4` | `x`, `y`, `z`, `w` | Homogeneous coordinates, colors |

All three are `Copy`, `PartialEq`, and `Default` (defaults to zero), and all
of them enforce finite components.

Examples below assume they live inside a function returning
`Result<_, Box<dyn std::error::Error>>` (or any error type that accepts
`Vec3Error`), so `?` works.

## Construction

Construction is fallible because `NaN` and infinity are rejected:

```rust
use plam::Vec3;

let v = Vec3::new(1.0, 2.0, 3.0)?;   // Result<Vec3, Vec3Error>
let s = Vec3::splat(2.0)?;            // (2, 2, 2) — splat exists on Vec2, Vec3, and Vec4
```

### Constants

```rust
use plam::{Vec2, Vec3, Vec4};

let a = Vec2::ZERO;   // also: ONE, UP, DOWN, LEFT, RIGHT
let b = Vec3::UP;     // also: ZERO, ONE, DOWN, LEFT, RIGHT, FORWARD, BACKWARD
let c = Vec4::ONE;    // also: ZERO, UP, DOWN, LEFT, RIGHT, FORWARD, BACKWARD
```

`Vec3::FORWARD` / `Vec4::FORWARD` point along −Z; `BACKWARD` along +Z.

## Reading components

Fields are private to protect invariants. Use the accessors:

```rust
use plam::Vec3;

let v = Vec3::new(1.0, 2.0, 3.0)?;

let x = v.x();               // const fn — works in const contexts too
let y = v.y();
let z = v.z();

let [x, y, z] = v.to_array();
```

All three provide `to_array()`; `Vec4` additionally provides `w()`.

## Arithmetic

Binary operators are implemented and **return `Result`**:

```rust
use plam::Vec3;

let a = Vec3::new(1.0, 2.0, 3.0)?;
let b = Vec3::new(4.0, 5.0, 6.0)?;

let sum = (a + b)?;                   // (5, 7, 9)
let diff = (a - b)?;                  // (-3, -3, -3)
let hadamard = (a * b)?;              // component-wise product
let quotient = (b / a)?;              // component-wise division
let scaled = (a * 2.0)?;              // scalar multiply
let halved = (b / 2.0)?;              // scalar divide
let negated = (-a)?;                  // negation
```

Two consequences of the `Result` output:

1. **Chain with parentheses and `?` (or `unwrap`/`match`).**
   `a + b + c` does not compile on its own, because the first `+` already
   yields `Result<Vec3, Vec3Error>`:

   ```rust
   use plam::{Vec3, Vec3Error};

   fn sum3(a: Vec3, b: Vec3, c: Vec3) -> Result<Vec3, Vec3Error> {
       Ok(((a + b)? + c)?)
   }
   ```

2. **Scalar on the left is not implemented.** Write `v * 2.0`, not `2.0 * v`.

Division checks for zero components and returns `Vec2Error::DivisionByZero`
(or the `Vec3`/`Vec4` equivalent) instead of producing infinity.

## Geometry helpers

| Method | Returns | Notes |
|---|---|---|
| `length_squared()` | `Result<f32, _>` | Squared length, no square root |
| `length()` | `Result<f32, _>` | Scaled to avoid overflow on large components |
| `dot(other)` | `Result<f32, _>` | Dot product |
| `distance(other)` | `Result<f32, _>` | Euclidean distance |
| `normalize()` | `Result<Self, _>` | `Vec2`, `Vec3`, and `Vec4` |
| `cross(other)` | `Result<Self, _>` | `Vec3` only |
| `lerp(other, t)` | `Result<Self, _>` | Linear interpolation; `t` outside `[0, 1]` extrapolates |
| `perpendicular()` | `Result<Self, _>` | `Vec2` only: rotates 90° counter-clockwise |
| `abs()`, `min(other)`, `max(other)` | `Result<Self, _>` | `Vec4` only |
| `xyz()` | `Result<Vec3, _>` | `Vec4` and `Quat` → first three components |

`dot` and `lerp` accumulate in `f64` internally before narrowing back to
`f32`, which avoids intermediate overflow and double rounding.

Normalizing a zero-length vector fails with `ZeroLength`:

```rust
use plam::Vec3;

match Vec3::ZERO.normalize() {
    Ok(unit) => println!("unit: {unit:?}"),
    Err(e) => println!("cannot normalize: {e}"),   // Cannot normalize a zero-length vector
}
```

## Interpolation

`lerp(other, t)` computes the linear interpolation
`self + (other - self) * t` (available on `Vec2`, `Vec3`, and `Vec4`):

```rust
use plam::Vec3;

let start = Vec3::ZERO;
let end = Vec3::new(10.0, 20.0, 30.0)?;

let half = start.lerp(end, 0.5)?;    // (5, 10, 15)
let beyond = start.lerp(end, 1.5)?;  // (15, 30, 45) — extrapolation is allowed
```

`t` must be finite (`NonFiniteInput` otherwise); results that overflow to
`NaN`/infinity fail with `InvalidResult`.

## Queries

```rust
use plam::Vec3;

let v = Vec3::new(1.0, 0.0, 0.0)?;
let finite = v.is_finite();   // false only if a component is NaN/±inf
let empty = v.is_zero();      // exact comparison with 0.0
```

Vectors are valid-by-construction, so `is_finite()` is mainly useful for
values you kept around across FFI or unsafe boundaries.

## Serialization

With the `serde` feature (default):

```rust
use plam::Vec3;

// serde_json is not a dependency of plam; illustrative only
let value = Vec3::new(1.0, 2.0, 3.0)?;
let json = serde_json::to_string(&value)?;
let back: Vec3 = serde_json::from_str(&json)?;
```

Serializing a non-finite vector fails; deserializing validates through
`Vec3::new`, so hand-edited data cannot smuggle `NaN` into your scene.
