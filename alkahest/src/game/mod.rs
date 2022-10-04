use super::*;
use glfw::Context;

pub(super) fn sys_init() -> util::context::Context {
    println!("In game::sys_init()!");
    util::context::Context::init().unwrap()
}

pub(super) fn sys_update(window_context: &mut util::types::WindowContext) {
    window::process_events(window_context);
    window_context.window.swap_buffers();
}

pub(super) fn sys_cleanup() {
    println!("In game::sys_cleanup()!");
}
