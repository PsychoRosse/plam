mod mat4_error;
mod mouse_delta_error;
mod mouse_scroll_delta_error;

pub use {
    mat4_error::Mat4Error, mouse_delta_error::MouseDeltaError,
    mouse_scroll_delta_error::MouseScrollDeltaError,
};
