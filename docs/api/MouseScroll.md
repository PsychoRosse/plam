# MouseScroll

A mouse scroll event, distinguishing line-based wheels from pixel-based
trackpads.

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{MouseScroll, MouseScrollDelta};

let lines = MouseScrollDelta::new(0.0, -3.0)?;   // wheel moved 3 lines
let event = MouseScroll::Lines(lines);           // or MouseScroll::Pixels(...)
# Ok(())
# }
```

Variants:

- `Lines(MouseScrollDelta)` — detent-based wheels, measured in lines
- `Pixels(MouseScrollDelta)` — smooth scrolling, measured in pixels

Both carry a [`MouseScrollDelta`] with the same finite/range rules as
[`MouseDelta`]. With default features the enum derives `clama::Event` and
`Serialize`/`Deserialize`.

Repository guide: `docs/window-and-input.md`.

[`MouseScrollDelta`]: crate::MouseScrollDelta
[`MouseDelta`]: crate::MouseDelta
