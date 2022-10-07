#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod input_impl;
pub use input_impl::{Key, MouseButton};

static mut KEY_STATE: [bool; 121] = [false; 121];
static mut MOUSE_BUTTON_STATE: [bool; 8] = [false; 8];
static mut MOUSE_POSITION: glm::Vec2 = glm::Vec2 { x: 0., y: 0. };
static mut MOUSE_SCROLL: glm::Vec2 = glm::Vec2 { x: 0., y: 0. };

/// Check if key is pressed
///
/// * k: Key - The key to query.
///
/// -> bool
pub fn is_key_down(k: Key) -> bool {
    unsafe {
        if (k as u32) < (KEY_STATE.len() as u32) {
            return KEY_STATE[k as usize];
        }
    }
    false
}

/// Check if key is up
///
/// * k: Key - The key to query.
///
/// -> bool
pub fn is_key_up(k: Key) -> bool {
    unsafe {
        if (k as u32) < (KEY_STATE.len() as u32) {
            return !KEY_STATE[k as usize];
        }
    }
    false
}

/// Check if mouse button is pressed
///
/// * mb: MouseButton - The mouse button to query.
///
/// -> bool
pub fn is_mouse_button_down(mb: MouseButton) -> bool {
    unsafe {
        if (mb as usize) < MOUSE_BUTTON_STATE.len() {
            return MOUSE_BUTTON_STATE[mb as usize];
        }
    }
    false
}

/// Check if mouse button is up
///
/// * mb: MouseButton - The mouse button to query.
///
/// -> bool
pub fn is_mouse_button_up(mb: MouseButton) -> bool {
    unsafe {
        if (mb as usize) < MOUSE_BUTTON_STATE.len() {
            return !MOUSE_BUTTON_STATE[mb as usize];
        }
    }
    false
}

/// Returns current mouse position
///
/// -> glm::Vec2
pub fn get_mouse_pos<'a>() -> &'a glm::Vec2 {
    unsafe {
        return &MOUSE_POSITION;
    }
}

/// Returns current mouse scroll
///
/// -> glm::Vec2
pub fn get_mouse_scroll<'a>() -> &'a glm::Vec2 {
    unsafe {
        return &MOUSE_SCROLL;
    }
}

pub(crate) fn set_key(k: u32, state: bool) {
    unsafe {
        if (k as usize) < KEY_STATE.len() {
            KEY_STATE[k as usize] = state;
        }
    }
}

pub(crate) fn set_mouse_button(mb: u32, state: bool) {
    unsafe {
        if (mb as usize) < MOUSE_BUTTON_STATE.len() {
            MOUSE_BUTTON_STATE[mb as usize] = state;
        }
    }
}

/// Sets mouse position
/// * x: f32 - X position.
/// * y: f32 - Y position.
pub fn set_mouse_pos(x: f32, y: f32) {
    unsafe {
        MOUSE_POSITION.x = x;
        MOUSE_POSITION.y = y;
    }
}

pub(crate) fn set_mouse_scroll(x: f32, y: f32) {
    unsafe {
        MOUSE_SCROLL.x = x;
        MOUSE_SCROLL.y = y;
    }
}
