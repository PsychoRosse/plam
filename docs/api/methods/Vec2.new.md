Creates a vector, rejecting non-finite components.

Returns [`Vec2Error::NonFiniteInput`](crate::Vec2Error) if any argument
is `NaN` or infinite. `const fn`.
