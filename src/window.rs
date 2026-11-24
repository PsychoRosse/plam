mod extent;
mod mouse_delta;
mod mouse_delta_error;
mod mouse_position;
mod mouse_position_error;
mod mouse_scroll;
mod mouse_scroll_delta;
mod mouse_scroll_delta_error;

pub use {
    extent::Extent, mouse_delta::MouseDelta, mouse_delta_error::MouseDeltaError,
    mouse_position::MousePosition, mouse_position_error::MousePositionError,
    mouse_scroll::MouseScroll, mouse_scroll_delta::MouseScrollDelta,
    mouse_scroll_delta_error::MouseScrollDeltaError,
};
