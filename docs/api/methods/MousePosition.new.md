Creates a position, rejecting non-finite components and values outside
`±MAX_COMPONENT` (`1_000_000.0`).

Errors are reported as [`MouseDeltaError`]. `const fn`.

[`MouseDeltaError`]: crate::MouseDeltaError
