Creates a right-handed view matrix looking from `eye` towards `center`
with the given up direction.

`up` does not need to be normalized. Degenerate configurations
(`eye == center`, or `up` parallel to the view direction) fail with
`InvalidLookAtParameters`.
