Creates a size with each side clamped to `1..=MAX_DIMENSION` and the
total area clamped to `MAX_AREA`.

Infallible by design: out-of-range input is adjusted, not rejected.
