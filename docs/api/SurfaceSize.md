# SurfaceSize

A render-surface size with non-zero dimensions — structurally the same
guarantees as [`WindowSize`], plus an aspect-ratio helper.

```rust
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use plam::{Mat4, SurfaceSize};

let surface = SurfaceSize::clamped(1920, 1080);
let aspect = surface.aspect_ratio();

let projection = Mat4::perspective_rh(60.0, aspect, 0.1, 100.0)?;
# Ok(())
# }
```

| Item | Description |
|---|---|
| `clamped(width, height)` | Same clamping rules as [`WindowSize`] |
| `minimum()` | 1 × 1 |
| `Default` | 1280 × 720 |
| `width()` / `height()` / `area()` | Dimensions and area (`const fn`) |
| `aspect_ratio()` | `width / height` as `f32` (`const fn`) — ready for `perspective_rh` |
| `with_width(w)` / `with_height(h)` | Replace one dimension, then clamp |
| `is_landscape()` / `is_portrait()` / `is_square()` | Orientation queries (`const fn`) |
| `min_dimension()` / `max_dimension()` | Smaller / larger side (`const fn`) |
| `MAX_DIMENSION` / `MAX_AREA` | `16_384` per side, `67_108_864` pixels |

Implements `From<[u32; 2]>` / `From<(u32, u32)>` (clamping) and
`Serialize`/`Deserialize` (clamping) with the `serde` feature; derives
`clama::Event` with the `clama` feature.

Repository guide: `docs/window-and-input.md`.

[`WindowSize`]: crate::WindowSize
