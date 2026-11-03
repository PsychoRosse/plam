Rotates `vector` by this quaternion.

Normalizes `self` first, so non-unit quaternions rotate correctly.
Computes in `f64`. Fails with `NonFiniteInput`, `ZeroLength`, or
`InvalidResult`.
