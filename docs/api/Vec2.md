# Vec2

A 2D vector with `f32` components — positions, directions, velocities, UV
coordinates.

## Example

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::Vec2;

let a = Vec2::new(1.0, 2.0)?;
let b = Vec2::splat(3.0)?;

let sum = (a + b)?;                       // operators return Result
let length = a.length()?;
let direction = Vec2::new(3.0, 4.0)?.normalize()?;
let halfway = a.lerp(b, 0.5)?;
# Ok(())
# }
```

## Key facts

- **Valid by construction:** `new` rejects `NaN`/infinity; fields are
  private, read them with `x()`, `y()`, or `to_array()` (all `const fn`).
- **Fallible operators:** `+`, `-`, `*`, `/`, and unary `-` return
  `Result<Vec2, Vec2Error>`. Chain as `((a + b)? + c)?`; write `v * 2.0`,
  not `2.0 * v`.
- **Constants:** [`Vec2::ZERO`], [`Vec2::ONE`], [`Vec2::UP`],
  [`Vec2::DOWN`], [`Vec2::LEFT`], [`Vec2::RIGHT`].
- `Copy`, `PartialEq`, `Default` (defaults to zero).

## Methods

| Method | Description |
|---|---|
| `new(x, y)` / `splat(value)` | Fallible construction (`const fn`) |
| `x()` / `y()` / `to_array()` | Component access (`const fn`) |
| `is_finite()` / `is_zero()` | Queries (`const fn`) |
| `length_squared()` / `length()` | Magnitude; `length` is scaled to resist overflow |
| `dot(other)` | Dot product, accumulated in `f64` |
| `normalize()` | Unit vector; fails with `ZeroLength` on a zero vector |
| `distance(other)` | Euclidean distance |
| `lerp(other, t)` | Linear interpolation in `f64`; `t` outside `[0, 1]` extrapolates |
| `perpendicular()` | 90° counter-clockwise rotation, `-y, x` |
| `cross` | Not applicable in 2D — use `perpendicular` |

`length()`, `normalize()`, and `distance()` use `sqrt` and are therefore
not `const`; everything else in the table is.

## Errors

[`Vec2Error`]: `NonFiniteInput`, `InvalidResult`, `DivisionByZero`,
`ZeroLength`. See the crate-level error handling guide in `docs/errors.md`.

## See also

Repository guides: `docs/vectors.md`.
