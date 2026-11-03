# MouseDeltaError

Errors returned by [`MouseDelta`] and [`MousePosition`] operations.
Implements `std::error::Error` (via `thiserror`),
`Copy + PartialEq + Eq + Hash`, and is serializable with the `serde`
feature.

| Variant | When |
|---|---|
| `NonFinite` | A component is `NaN` / `±inf` |
| `OutOfRange` | A component exceeds `MAX_COMPONENT` (`1_000_000.0`) |

[`MouseDelta`]: crate::MouseDelta
[`MousePosition`]: crate::MousePosition
