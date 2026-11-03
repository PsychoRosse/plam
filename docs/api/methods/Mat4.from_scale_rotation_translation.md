Creates an SRT matrix: scale first, then rotation, then translation.

Equivalent to `translation * rotation * scale`. Fails when any input is
non-finite or the quaternion cannot be normalized.
