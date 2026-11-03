Builds a rotation from a scaled-axis vector.

The magnitude of `scaled_axis` is a rotation angle in **degrees** around
its direction; a zero vector yields [`Quat::IDENTITY`]. Note: some
libraries treat this magnitude as radians.
