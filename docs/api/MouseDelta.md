# MouseDelta

A bounded 2D mouse movement delta. Components must be finite and within
`±MAX_COMPONENT` (`1_000_000.0`).

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{MouseDelta, MousePosition};

let from = MousePosition::new(100.0, 100.0)?;
let to = MousePosition::new(140.0, 160.0)?;
let delta = from.checked_sub(to)?;              // MouseDelta

let accumulated = MouseDelta::new(1.0, 2.0)?
    .checked_add(delta)?;
# Ok(())
# }
```

| Item | Description |
|---|---|
| `new(x, y)` | Fallible construction (`const fn`) |
| `x()` / `y()` / `is_valid()` | Accessors and bounds check (`const fn`) |
| `checked_add` / `checked_add_assign` | Accumulate deltas with range re-checks (`const fn`) |
| `DEFAULT_MULTIPLIER` | `32.0` scaling convenience constant |
| `MAX_COMPONENT` | `1_000_000.0` component bound |

[`TryFrom<MousePosition>`] converts a position into a delta. Errors are
[`MouseDeltaError`]. With default features the type derives
`clama::Event` and implements `Serialize`/`Deserialize` with validation.

Repository guide: `docs/window-and-input.md`.
