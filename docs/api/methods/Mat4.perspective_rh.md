Creates a right-handed perspective projection with clip depth `[-1, 1]`
(OpenGL style).

`fov_y_degrees` is the vertical field of view; `aspect_ratio` is
width / height; `near > 0` and `far > near`. Invalid parameters fail with
`InvalidPerspectiveParameters`. Uses `tan`, so it is not `const`.
