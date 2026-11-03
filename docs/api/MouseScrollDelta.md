# MouseScrollDelta

A bounded 2D scroll delta. Components must be finite and within
`±MAX_COMPONENT` (`1_000_000.0`).

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::MouseScrollDelta;

let scroll = MouseScrollDelta::new(0.0, -3.0)?;
let (x, y) = (scroll.x(), scroll.y());
# Ok(())
# }
```

| Item | Description |
|---|---|
| `new(x, y)` | Fallible construction (`const fn`) |
| `x()` / `y()` | Accessors (`const fn`) |
| `DEFAULT_MULTIPLIER` | `32.0` scaling convenience constant |
| `MAX_COMPONENT` | `1_000_000.0` component bound |

Errors are [`MouseScrollDeltaError`]. Carried by [`MouseScroll::Lines`]
and [`MouseScroll::Pixels`]. With default features the type derives
`clama::Event` and implements `Serialize`/`Deserialize` with validation.

Repository guide: `docs/window-and-input.md`.

[`MouseScrollDeltaError`]: crate::MouseScrollDeltaError
[`MouseScroll::Lines`]: crate::MouseScroll::Lines
[`MouseScroll::Pixels`]: crate::MouseScroll::Pixels
