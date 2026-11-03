Linear interpolation towards `other` by factor `t`:
`self + (other - self) * t`.

Computed in `f64`. `t` outside `[0, 1]` extrapolates; a non-finite `t`
fails with `NonFiniteInput`, an overflowing result with `InvalidResult`.
`const fn`.
