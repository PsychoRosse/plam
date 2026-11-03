# MousePosition

A bounded 2D mouse position. Components must be finite and within
`±MAX_COMPONENT` (`1_000_000.0`).

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::MousePosition;

let a = MousePosition::new(10.0, 20.0)?;
let b = MousePosition::new(4.0, 6.0)?;
let delta = a.checked_sub(b)?;      // MouseDelta (6, 14)
# Ok(())
# }
```

| Item | Description |
|---|---|
| `new(x, y)` | Fallible construction (`const fn`) |
| `x()` / `y()` | Accessors (`const fn`) |
| `checked_sub(other)` | Difference as a [`MouseDelta`] (`const fn`) |
| `MAX_COMPONENT` | `1_000_000.0` component bound |

Errors are reported as [`MouseDeltaError`] (`NonFinite` / `OutOfRange`
apply verbatim). With default features the type derives `clama::Event`
and implements `Serialize`/`Deserialize` with validation.

Repository guide: `docs/window-and-input.md`.

[`MouseDelta`]: crate::MouseDelta
[`MouseDeltaError`]: crate::MouseDeltaError
