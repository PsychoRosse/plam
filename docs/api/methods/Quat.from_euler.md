Builds a rotation from Euler angles in **degrees** (X, Y, Z order).

Angles are wrapped with `rem_euclid(360.0)`, so any finite value is
accepted. Fails with `NonFiniteInput` for non-finite arguments.
