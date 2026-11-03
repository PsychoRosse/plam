# Prelude

Glob-import every public type and error:

```rust
use plam::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v = Vec3::new(1.0, 2.0, 3.0)?;
    let q = Quat::from_rotation_y(90.0)?;
    Ok(())
}
```

Re-exports: [`Vec2`], [`Vec3`], [`Vec4`], [`Quat`], [`Mat4`], their error
types ([`Vec2Error`], [`Vec3Error`], [`Vec4Error`], [`QuatError`],
[`Mat4Error`]), and the window/input types ([`WindowSize`], [`SurfaceSize`],
[`MousePosition`], [`MouseDelta`], [`MouseScroll`], [`MouseScrollDelta`],
[`MouseDeltaError`], [`MouseScrollDeltaError`]).

[`Vec2`]: crate::Vec2
[`Vec3`]: crate::Vec3
[`Vec4`]: crate::Vec4
[`Quat`]: crate::Quat
[`Mat4`]: crate::Mat4
[`Vec2Error`]: crate::Vec2Error
[`Vec3Error`]: crate::Vec3Error
[`Vec4Error`]: crate::Vec4Error
[`QuatError`]: crate::QuatError
[`Mat4Error`]: crate::Mat4Error
[`WindowSize`]: crate::WindowSize
[`SurfaceSize`]: crate::SurfaceSize
[`MousePosition`]: crate::MousePosition
[`MouseDelta`]: crate::MouseDelta
[`MouseScroll`]: crate::MouseScroll
[`MouseScrollDelta`]: crate::MouseScrollDelta
[`MouseDeltaError`]: crate::MouseDeltaError
[`MouseScrollDeltaError`]: crate::MouseScrollDeltaError
