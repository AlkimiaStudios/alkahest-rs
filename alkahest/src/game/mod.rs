use super::*;
use glfw::Context;

pub(super) fn sys_init() -> util::context::Context {
    trace!("Initializing engine context");
    match util::context::Context::init() {
        Err(e) => panic!("Unable to initialize engine context! Error: {:?}", e),
        Ok(context) => context,
    }
}

pub(super) fn sys_update(window_context: &mut util::types::WindowContext) {
    window::process_events(window_context);
    window_context.window.swap_buffers();
}

pub(super) fn sys_cleanup() {
    trace!("In game::sys_cleanup()!");
}
