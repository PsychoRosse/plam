Returns the multiplicative inverse `conjugate / |q|²` for any non-zero
quaternion.

The norm is computed in `f64`. Fails with `ZeroLength` for a zero
quaternion. `const fn`.
