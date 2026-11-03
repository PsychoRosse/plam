Returns the unit-length vector in the same direction.

Fails with `ZeroLength` for a zero vector and `NonFiniteInput` for
non-finite components. Uses `sqrt`, so it is not `const`.
