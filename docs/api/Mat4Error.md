# Mat4Error

Errors returned by [`Mat4`] operations. Implements `std::error::Error`
(via `thiserror`), `Copy + PartialEq + Eq + Hash`, and is serializable
with the `serde` feature.

| Variant | When |
|---|---|
| `NonFiniteInput` | Any matrix or operand component is `NaN` / `±inf` |
| `SingularMatrix` | `inverse()` hit a zero pivot |
| `InvalidPerspectiveParameters` | Bad FOV / aspect / near / far |
| `InvalidOrthographicParameters` | Degenerate left/right, bottom/top, or near/far ranges |
| `InvalidLookAtParameters` | `eye == center`, or `up` parallel to the view direction |
| `InvalidQuaternion` | `from_quat` could not normalize the rotation |
| `ZeroHomogeneousCoordinate` | `transform_point3` divided by `w == 0` |
| `InvalidResult` | Overflow or non-finite result |

See `docs/errors.md` for handling patterns.

[`Mat4`]: crate::Mat4
