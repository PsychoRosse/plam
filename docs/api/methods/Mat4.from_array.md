Creates a matrix from 16 floats in **column-major** order
(the same order as [`to_array`](Self::to_array)).

Fails with `NonFiniteInput` if any value is not finite. `const fn`.
