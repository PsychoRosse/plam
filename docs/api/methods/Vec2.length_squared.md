Returns the squared length — `x² + y²` without a square root.

Fails with `NonFiniteInput` on non-finite self and `InvalidResult` if the
sum overflows. Prefer it over [`length`](Self::length) when only comparing
magnitudes. `const fn`.
