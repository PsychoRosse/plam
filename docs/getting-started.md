# Getting started

## Installation

Add Plam to your `Cargo.toml`:

```toml
[dependencies]
plam = "0.3"
```

Plam targets Rust edition 2024.

## Features

| Feature | Default | Description |
|---|---|---|
| `serde` | yes | `Serialize` / `Deserialize` for all public types, with validation on both directions |
| `clama` | yes | Derives `clama::Event` for the window and input types |

To opt out of the defaults:

```toml
[dependencies]
plam = { version = "0.3", default-features = false }
```

With `serde` enabled, serializing a non-finite vector fails with an error
instead of writing invalid data, and deserializing runs the same validation
as the constructors.

## Prelude

If you prefer shorter imports:

```rust
use plam::prelude::*;
```

The prelude re-exports every type and error: `Vec2`, `Vec3`, `Vec4`, `Quat`,
`Mat4`, the `*Error` types, and the window/input types.

## First example

Every constructor and operation that can mathematically fail returns a
`Result`. Handle it with `?`, `match`, or an explicit fallback of your choice:

```rust
use plam::{Mat4, Quat, Vec3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vectors
    let position = Vec3::new(1.0, 2.0, 3.0)?;
    let direction = Vec3::new(0.0, 3.0, 4.0)?;
    let forward = direction.normalize()?;

    // Rotation: angles are in degrees
    let rotation = Quat::from_rotation_y(90.0)?;
    let rotated = rotation.rotate_vec3(forward)?;

    // A model matrix from scale, rotation, and translation
    let model = Mat4::from_scale_rotation_translation(
        Vec3::new(1.0, 1.0, 1.0)?,
        rotation,
        position,
    )?;

    let point = model.transform_point3(Vec3::ZERO)?;
    println!("{point:?} rotated to {rotated:?}");

    Ok(())
}
```

## Where to go next

- [Vectors](vectors.md) — component access, arithmetic, geometry helpers.
- [Quaternions](quaternions.md) — building and composing rotations.
- [Matrices](matrices.md) — transforms, inversion, projections.
- [Error handling](errors.md) — what each error means and how to react.
