Returns the determinant, computed in `f64` with partial pivoting.

Returns `0.0` for singular matrices; fails with `InvalidResult` when the
value cannot be represented as `f32`. `const fn`.
