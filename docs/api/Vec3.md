# Vec3

A 3D vector with `f32` components — positions, directions, velocities.

## Example

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::Vec3;

let a = Vec3::new(1.0, 2.0, 3.0)?;
let b = Vec3::new(4.0, 5.0, 6.0)?;

let sum = (a + b)?;
let cross = a.cross(b)?;
let unit = Vec3::new(0.0, 3.0, 4.0)?.normalize()?;
let halfway = Vec3::lerp(Vec3::ZERO, Vec3::ONE, 0.5)?;
# Ok(())
# }
```

## Key facts

- **Valid by construction:** `new` rejects `NaN`/infinity; read components
  with `x()`, `y()`, `z()`, or `to_array()` (all `const fn`).
- **Fallible operators:** `+`, `-`, `*`, `/`, and unary `-` return
  `Result<Vec3, Vec3Error>`. Chain as `((a + b)? + c)?`; write `v * 2.0`,
  not `2.0 * v`.
- **Constants:** [`Vec3::ZERO`], [`Vec3::ONE`], [`Vec3::UP`],
  [`Vec3::DOWN`], [`Vec3::LEFT`], [`Vec3::RIGHT`], [`Vec3::FORWARD`
  (−Z)](crate::Vec3::FORWARD), [`Vec3::BACKWARD` (+Z)](crate::Vec3::BACKWARD).
- `Copy`, `PartialEq`, `Default` (defaults to zero).

## Methods

| Method | Description |
|---|---|
| `new(x, y, z)` / `splat(value)` | Fallible construction (`const fn`) |
| `x()` / `y()` / `z()` / `to_array()` | Component access (`const fn`) |
| `is_finite()` / `is_zero()` | Queries (`const fn`) |
| `length_squared()` / `length()` | Magnitude; `length` is scaled to resist overflow |
| `dot(other)` | Dot product, accumulated in `f64` (`const fn`) |
| `cross(other)` | Right-handed cross product (`const fn`) |
| `normalize()` | Unit vector; fails with `ZeroLength` on a zero vector |
| `distance(other)` | Euclidean distance |
| `lerp(other, t)` | Linear interpolation in `f64`; `t` outside `[0, 1]` extrapolates (`const fn`) |

`length()`, `normalize()`, and `distance()` use `sqrt` and are therefore
not `const`; the rest of the table is.

## Errors

[`Vec3Error`]: `NonFiniteInput`, `InvalidResult`, `DivisionByZero`,
`ZeroLength`. See the crate-level error handling guide in `docs/errors.md`.

## See also

Repository guides: `docs/vectors.md`.
