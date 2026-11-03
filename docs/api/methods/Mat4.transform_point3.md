Transforms a point (`w = 1`) and performs the homogeneous divide by `w`.

Fails with `ZeroHomogeneousCoordinate` when the resulting `w` is `0`, and
with `NonFiniteInput` / `InvalidResult` on invalid data.
