# Vec4Error

Errors returned by [`Vec4`] operations. Implements `std::error::Error`
(via `thiserror`), `Copy + PartialEq + Eq + Hash`, and is serializable
with the `serde` feature.

| Variant | When |
|---|---|
| `NonFiniteInput` | A component or argument is `NaN` / `±inf` |
| `InvalidResult` | The operation overflowed or produced a non-finite value |
| `DivisionByZero` | Division by a zero component or scalar |
| `ZeroLength` | Normalizing a zero-length vector |

See `docs/errors.md` for handling patterns.

[`Vec4`]: crate::Vec4
