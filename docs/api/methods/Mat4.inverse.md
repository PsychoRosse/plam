Returns the inverse via Gauss-Jordan elimination with partial pivoting
(computed in `f64`).

Fails with `SingularMatrix` when a pivot is zero. `const fn`.
