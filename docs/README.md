# Plam documentation

Guides for [Plam](../README.md), a safe mathematics library for 2D and 3D game development.

## Contents

| Guide | Description |
|---|---|
| [Getting started](getting-started.md) | Installation, features, prelude, first example |
| [Vectors](vectors.md) | `Vec2`, `Vec3`, `Vec4`: construction, accessors, arithmetic |
| [Quaternions](quaternions.md) | `Quat`: rotations, Euler angles, axis-angle, degrees convention |
| [Matrices](matrices.md) | `Mat4`: layout, transforms, inversion, perspective projection |
| [Error handling](errors.md) | The safe-math model, error types, handling patterns |
| [Window and input types](window-and-input.md) | `WindowSize`, `SurfaceSize`, mouse position, delta, and scroll |

## API reference

The [`api/`](api/lib.md) folder holds per-type markdown that is attached
to the source with `#[doc = include_str!]`; [`api/methods/`](api/methods)
does the same for every inherent public method (trait implementations are
documented by their types). Running

```text
cargo doc --no-deps --open
```

renders it as the full rustdoc API reference, and `cargo test` compiles
and runs every example in those files as a doctest.

## Conventions used throughout

- **Angles are degrees.** Every API that takes or returns an angle
  (`Quat::from_euler`, `Quat::from_axis_angle`, `Mat4::perspective_rh`, …)
  uses degrees, not radians.
- **Fallible operations return `Result`.** Construction, arithmetic,
  normalization, inversion, and projection can all fail; see
  [Error handling](errors.md).
- **Fields are private.** Read components through the `x()`, `y()`, `z()`,
  `w()` accessors or `to_array()`.
- **Matrices are column-major.** `Mat4` stores four column vectors
  (`x_axis` … `w_axis`); see [Matrices](matrices.md).
- **Many methods are `const fn`.** Construction, queries, and pure
  arithmetic (`dot`, `lerp`, `cross`, `transpose`, `inverse`,
  `orthographic_rh`, …) work in const contexts. Methods that need
  `sqrt` or trigonometry (`length`, `normalize`, `distance`,
  quaternion rotation constructors, `perspective_rh`) are runtime-only,
  because those float operations are not `const` in Rust yet.
