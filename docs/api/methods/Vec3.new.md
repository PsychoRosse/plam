Creates a vector, rejecting non-finite components.

Returns [`Vec3Error::NonFiniteInput`](crate::Vec3Error) if any argument
is `NaN` or infinite. `const fn`.
