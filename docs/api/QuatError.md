# QuatError

Errors returned by [`Quat`] operations. Implements `std::error::Error`
(via `thiserror`), `Copy + PartialEq + Eq + Hash`, and is serializable
with the `serde` feature.

| Variant | When |
|---|---|
| `NonFiniteInput` | A component or argument is `NaN` / `±inf` |
| `InvalidResult` | The operation overflowed or produced a non-finite value |
| `DivisionByZero` | Division by a zero scalar |
| `ZeroLength` | Normalizing or inverting a zero quaternion |
| `InvalidAxis` | `from_axis_angle` received a zero axis |

See `docs/errors.md` for handling patterns.

[`Quat`]: crate::Quat
