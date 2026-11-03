# Window and input types

Besides the math primitives, Plam ships a small set of window/input value
types with the same validity guarantees. They are useful for event-driven
UIs and game loops that want checked values at API boundaries.

> These types are not part of the core math API. If you only need vectors,
> quaternions, and matrices, you can ignore this page.

## WindowSize and SurfaceSize

`WindowSize` and `SurfaceSize` are structurally identical: a non-zero
width and height (`NonZero<u32>`), so a zero-sized window cannot exist.

```rust
use plam::{WindowSize, SurfaceSize};

let window = WindowSize::fhd();          // 1920 × 1080, Default is hd(): 1280 × 720
let surface = SurfaceSize::clamped(800, 600);

println!("{} × {}", window.width(), window.height());
println!("area = {}", window.area());
println!("aspect = {:.2}", surface.aspect_ratio());
```

### Clamping instead of errors

Both types **clamp** rather than reject:

- Each dimension is clamped to `1..=16_384` (`MAX_DIMENSION`).
- The total area is limited to `67_108_864` pixels (`MAX_AREA` = 64 MP);
  oversize sizes are scaled down proportionally and then nudged until they
  fit.

```rust
use plam::WindowSize;

let huge = WindowSize::clamped(100_000, 100_000);   // shrunk to fit MAX_AREA
let zero = WindowSize::clamped(0, 0);               // becomes 1 × 1
```

`From<[u32; 2]>` and `From<(u32, u32)>` use the same clamping, so these
conversions are infallible.

### Presets and helpers

| API | Value / meaning |
|---|---|
| `minimum()` | 1 × 1 |
| `hd()` / `fhd()` / `qhd()` / `uhd()` | 1280×720, 1920×1080, 2560×1440, 3840×2160 |
| `Default` | `hd()` for `WindowSize`, 1280×720 for `SurfaceSize` |
| `with_width(w)` / `with_height(h)` | Replace one dimension (then clamp) |
| `area()` | `width * height` as `u64` |
| `is_landscape()` / `is_portrait()` / `is_square()` | Orientation queries |
| `min_dimension()` / `max_dimension()` | Smaller / larger side |

`SurfaceSize` additionally provides `aspect_ratio()` — handy for
`Mat4::perspective_rh`:

```rust
use plam::{Mat4, SurfaceSize};

fn projection_for(size: SurfaceSize) -> Result<Mat4, plam::Mat4Error> {
    Mat4::perspective_rh(60.0, size.aspect_ratio(), 0.1, 100.0)
}
```

## MousePosition and MouseDelta

```rust
use plam::{MouseDelta, MousePosition};

let a = MousePosition::new(100.0, 200.0)?;
let b = MousePosition::new(140.0, 260.0)?;

let moved = a.checked_sub(b)?;          // MouseDelta (40, 60)
let total = MouseDelta::new(1.0, 2.0)?.checked_add(moved)?;
```

- Components are finite and bounded by `MAX_COMPONENT` = `1_000_000.0`;
  anything larger fails with `OutOfRange`.
- `MousePosition::checked_sub` produces a `MouseDelta`;
  `MouseDelta::checked_add` / `checked_add_assign` accumulate deltas with
  range re-checks.
- `MousePosition::new` reports errors as `MouseDeltaError`
  (both variants — `NonFinite` / `OutOfRange` — apply verbatim).
- `TryFrom<MousePosition> for MouseDelta` is implemented.
- `MouseDelta::DEFAULT_MULTIPLIER` = `32.0` is a convenience constant for
  scroll/pixel scaling conventions.

## MouseScroll and MouseScrollDelta

```rust
use plam::{MouseScroll, MouseScrollDelta};

let scroll = MouseScrollDelta::new(0.0, -3.0)?;   // wheel moved 3 lines down
let event = MouseScroll::Lines(scroll);            // or MouseScroll::Pixels(...)
```

`MouseScroll` is an enum distinguishing line-based wheels from
pixel-based trackpads; both carry a `MouseScrollDelta`, which follows the
same finite/range rules as `MouseDelta`
(`MAX_COMPONENT` = `1_000_000.0`, `DEFAULT_MULTIPLIER` = `32.0`).

## Serde and clama

With default features these types:

- **`serde`:** serialize/deserialize with validation (mouse types) or
  clamping (sizes), so external input cannot create out-of-range values.
- **`clama`:** derive `clama::Event`, allowing them to be used directly
  as event payloads in the Clama framework.
