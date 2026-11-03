# MouseScrollDeltaError

Errors returned by [`MouseScrollDelta`] operations. Implements
`std::error::Error` (via `thiserror`), `Copy + PartialEq + Eq + Hash`,
and is serializable with the `serde` feature.

| Variant | When |
|---|---|
| `NonFinite` | A component is `NaN` / `±inf` |
| `OutOfRange` | A component exceeds `MAX_COMPONENT` (`1_000_000.0`) |

[`MouseScrollDelta`]: crate::MouseScrollDelta
