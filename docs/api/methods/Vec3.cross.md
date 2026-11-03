Returns the right-handed cross product `self × other`.

For unit axes: `right × up = forward(+Z)`. Fails with `NonFiniteInput`
for non-finite operands and `InvalidResult` on overflow. `const fn`.
