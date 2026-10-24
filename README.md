# Math

A lightweight Rust library for mathematical operations in 2D and 3D graphics.

## Features

- Vector types: `Vec2`, `Vec3`, `Vec4`
- Quaternion rotations: `Quat`
- 4×4 transformation matrices: `Mat4`
- Vector and matrix arithmetic
- Rotation, translation, and scaling transformations
- Perspective projection
- Fallible operations with `Result`-based error handling
- Input validation and serialization support

## Design

The library emphasizes explicit error handling, finite-value validation, and predictable behavior. Fallible operations use the `try_*` naming convention and return `Result` instead of silently substituting invalid results.

Angles are expressed in degrees.
