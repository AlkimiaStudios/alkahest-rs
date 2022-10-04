use super::*;
use glfw::Context;

pub(super) fn sys_init() -> util::context::Context {
    println!("In game::sys_init()!");
    util::context::Context::init().unwrap()

}

pub(super) fn sys_update(gw: &mut util::types::WindowContext) {
    window::process_events(&mut gw.window, &mut gw.glfw, &mut gw.events);
    gw.window.swap_buffers();
}

pub(super) fn sys_cleanup() {
    println!("In game::sys_cleanup()!");
}
