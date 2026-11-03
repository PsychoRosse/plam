Creates a scroll delta, rejecting non-finite components and values
outside `±MAX_COMPONENT` (`1_000_000.0`).

Errors: `NonFinite`, `OutOfRange`. `const fn`.
