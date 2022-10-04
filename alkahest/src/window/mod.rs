#[cfg_attr(target_os = "linux", path = "linux.rs")]
mod window_impl;

use super::util::types::WindowContext;

pub(crate) fn init(width: u32, height: u32, name: &str) -> Result<WindowContext, &'static str> {
    window_impl::init(width, height, name)
}

pub(crate) fn process_events(window_context: &mut WindowContext) {
    window_impl::process_events(window_context)
}
