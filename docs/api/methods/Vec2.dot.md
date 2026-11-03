Returns the dot product `self · other`.

Accumulated in `f64` before narrowing to `f32`, which avoids intermediate
overflow. Fails with `NonFiniteInput` or `InvalidResult`. `const fn`.
