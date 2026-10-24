# Plam

**Plam** is a safe mathematics library for Rust, designed for game development, 3D graphics, and geometric computations.

It provides essential mathematical primitives with explicit error handling, input validation, and predictable behavior.

## Features

- 2D and 3D vectors
- Quaternions for representing rotations
- 4×4 matrices for spatial transformations
- Vector and matrix arithmetic
- Rotation, translation, and scaling
- Perspective projection
- Serialization and deserialization
- Explicit error handling for fallible operations

## Core Types

### Vectors

Vectors represent positions, directions, velocities, and other multidimensional values.

- `Vec2` represents a 2D vector.
- `Vec3` represents a 3D vector.
- `Vec4` represents a 4D vector, including homogeneous coordinates used in graphics.

Example:

```rust
use plam::{Vec2, Vec3, Vec4};

let point = Vec2::new(2.0, 3.0);
let position = Vec3::new(1.0, 2.0, 3.0);
let homogeneous = Vec4::new(1.0, 2.0, 3.0, 1.0);
```

Vectors can be used for geometric calculations, spatial movement, and transformations.

### Quaternions

`Quat` represents rotations in 3D space. Quaternions are useful for composing rotations and avoiding some problems associated with Euler angles, such as gimbal lock.

```rust
use plam::{Quat, Vec3};

let rotation = Quat::from_rotation_y(90.0);
let axis = Vec3::UP;
```

Rotation angles are expressed in **degrees**.

Quaternions can be used to represent axis-angle rotations, Euler rotations, and scaled-axis rotations.

### Matrices

`Mat4` represents a 4×4 matrix, commonly used for transforming objects and points in 3D space.

Typical applications include:

- Translation
- Scaling
- Rotation
- Combining transformations
- Perspective projection
- Matrix inversion and transposition

```rust
use plam::{Mat4, Vec3};

let translation = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
let scale = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
```

## Safe Math

Plam aims to make mathematical failures explicit instead of silently producing invalid results.

Depending on the operation, invalid inputs or mathematically undefined results may include non-finite values, division by zero, normalization of a zero-length vector, or inversion of a singular matrix.

Fallible operations return `Result`, allowing errors to be handled explicitly.

```rust
use plam::Vec3;

fn normalize_direction(direction: Vec3) -> Result<Vec3, Box<dyn std::error::Error>> {
    let normalized = direction.normalize()?;
    Ok(normalized)
}
```

Handle errors when an operation can fail:

```rust
use plam::Vec3;

let direction = Vec3::new(0.0, 0.0, 0.0);

match direction.normalize() {
    Ok(normalized) => {
        println!("Normalized direction: {:?}", normalized);
    }
    Err(error) => {
        eprintln!("Cannot normalize vector: {error}");
    }
}
```

Avoid silently replacing errors with arbitrary fallback values. A zero vector, identity quaternion, or identity matrix should only be used when it is a valid result for the intended operation.

## Serialization

Plam supports serialization and deserialization with validation of numeric values.

Deserialized data should satisfy the library's validity requirements before being used in mathematical operations.

## Design Principles

- **Safety:** validate inputs and reject invalid numeric states.
- **Explicit errors:** fallible operations return `Result`.
- **Predictability:** avoid silently substituting valid-looking values when an operation fails.
- **Simplicity:** provide familiar mathematical primitives for graphics and game development.
