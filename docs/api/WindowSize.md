# WindowSize

A window size with non-zero dimensions. Out-of-range inputs are **clamped**
rather than rejected, so construction is infallible.

```rust
use plam::WindowSize;

let window = WindowSize::fhd();              // 1920 × 1080
let custom = WindowSize::clamped(800, 600);  // clamps to legal range
let rotated = window.with_width(1080);       // re-clamp one dimension

assert!(custom.is_landscape() || custom.is_portrait() || custom.is_square());
```

| Item | Description |
|---|---|
| `clamped(width, height)` | Clamp each side to `1..=MAX_DIMENSION` and the area to `MAX_AREA` |
| `minimum()` / `hd()` / `fhd()` / `qhd()` / `uhd()` | Presets: 1×1, 1280×720, 1920×1080, 2560×1440, 3840×2160 |
| `Default` | `hd()` (1280 × 720) |
| `width()` / `height()` / `area()` | Dimensions and area (`const fn`) |
| `with_width(w)` / `with_height(h)` | Replace one dimension, then clamp |
| `is_landscape()` / `is_portrait()` / `is_square()` | Orientation queries (`const fn`) |
| `min_dimension()` / `max_dimension()` | Smaller / larger side (`const fn`) |
| `MAX_DIMENSION` | `16_384` per side |
| `MAX_AREA` | `67_108_864` pixels (64 MP) |

Implements `From<[u32; 2]>` / `From<(u32, u32)>` (clamping) and
`Serialize`/`Deserialize` (clamping) with the `serde` feature; derives
`clama::Event` with the `clama` feature.

Repository guide: `docs/window-and-input.md`.
