# Vec4

A 4D vector with `f32` components — homogeneous coordinates, colors, and
general 4-component data.

## Example

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::Vec4;

let v = Vec4::new(1.0, 2.0, 3.0, 4.0)?;
let w = v.w();
let first_three = v.xyz()?;               // truncate to Vec3
let positive = v.abs()?;
let lo = v.min(Vec4::ONE)?;
let halfway = Vec4::lerp(Vec4::ZERO, Vec4::ONE, 0.5)?;
# Ok(())
# }
```

## Key facts

- **Valid by construction:** `new` rejects `NaN`/infinity; read components
  with `x()`, `y()`, `z()`, `w()`, or `to_array()` (all `const fn`).
- **Fallible operators:** `+`, `-`, `*`, `/`, and unary `-` return
  `Result<Vec4, Vec4Error>`. Chain as `((a + b)? + c)?`; write `v * 2.0`,
  not `2.0 * v`.
- **Constants:** [`Vec4::ZERO`], [`Vec4::ONE`], [`Vec4::UP`],
  [`Vec4::DOWN`], [`Vec4::LEFT`], [`Vec4::RIGHT`], [`Vec4::FORWARD`],
  [`Vec4::BACKWARD`] (direction constants have `w = 0`).
- `Copy`, `PartialEq`, `Default` (defaults to zero).

## Methods

| Method | Description |
|---|---|
| `new(x, y, z, w)` / `splat(value)` | Fallible construction (`const fn`) |
| `x()` / `y()` / `z()` / `w()` / `to_array()` | Component access (`const fn`) |
| `is_finite()` / `is_zero()` | Queries (`const fn`) |
| `length_squared()` / `length()` | Magnitude; `length` is scaled to resist overflow |
| `dot(other)` | Dot product, accumulated in `f64` (`const fn`) |
| `normalize()` | Unit vector; fails with `ZeroLength` on a zero vector |
| `distance(other)` | Euclidean distance |
| `lerp(other, t)` | Linear interpolation in `f64`; `t` outside `[0, 1]` extrapolates (`const fn`) |
| `abs()` / `min(other)` / `max(other)` | Component-wise helpers (`const fn`, `Vec4`-only) |
| `xyz()` | First three components as [`Vec3`] (`const fn`) |

`length()`, `normalize()`, and `distance()` use `sqrt` and are therefore
not `const`; the rest of the table is.

## Errors

[`Vec4Error`]: `NonFiniteInput`, `InvalidResult`, `DivisionByZero`,
`ZeroLength`. See the crate-level error handling guide in `docs/errors.md`.

## See also

Repository guides: `docs/vectors.md`.
