#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod window_impl;

use super::util::types::WindowContext;

/// Initializes the game window.
///
/// This function initializes the game window using whichever
/// graphics backend is available and supported.
///
/// * `width`: u32 - Width of the window.
/// * `height`: u32 - Height of the window.
/// * `name`: &str - Title of the window.
///
/// -> Result<alkahest::util::types::WindowContext, &'static str>
pub(crate) fn init(width: u32, height: u32, name: &str) -> Result<WindowContext, &'static str> {
    window_impl::init(width, height, name)
}

/// Processes events for the game window.
///
/// This function is called once per frame, and it processes any
/// events that have happened since the last call.
///
/// * `window_context`: &mut alkahest::types::WindowContext - The context object for the game window.
pub(crate) fn process_events(window_context: &mut WindowContext) {
    window_impl::process_events(window_context)
}
