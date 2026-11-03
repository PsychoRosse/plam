Creates a vector, rejecting non-finite components.

Returns [`Vec4Error::NonFiniteInput`](crate::Vec4Error) if any argument
is `NaN` or infinite. `const fn`.
