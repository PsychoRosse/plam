Creates a right-handed orthographic projection with clip depth `[-1, 1]`.

Requires finite inputs with `left != right`, `bottom != top`,
`near != far`; otherwise fails with `InvalidOrthographicParameters`.
`const fn`.
