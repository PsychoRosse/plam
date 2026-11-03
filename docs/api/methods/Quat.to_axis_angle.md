Returns the canonical `(axis, degrees)` pair for this rotation.

The angle lies in `[0, 180]` and `w` is kept non-negative, so the result
is unique. Fails when `self` cannot be normalized.
