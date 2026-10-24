mod mat4_error;
mod mouse_delta_error;
mod mouse_scroll_delta_error;
mod quat_error;
mod vec2_error;
mod vec3_error;
mod vec4_error;

pub use {
    mat4_error::Mat4Error, mouse_delta_error::MouseDeltaError,
    mouse_scroll_delta_error::MouseScrollDeltaError, quat_error::QuatError, vec2_error::Vec2Error,
    vec3_error::Vec3Error, vec4_error::Vec4Error,
};
