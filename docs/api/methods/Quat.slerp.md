Spherical linear interpolation towards `end` by factor `t`.

Both endpoints are normalized; the shortest arc is taken (negates `end`
when `dot < 0`); near-parallel inputs fall back to `lerp + normalize`.
`t` outside `[0, 1]` extrapolates. The result is always unit length.
Fails with `NonFiniteInput` or `ZeroLength`.
