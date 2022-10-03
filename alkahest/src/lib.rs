pub(crate) mod types;
pub(crate) mod context;
pub(crate) mod window;

mod game {
    use super::*;
    use glfw::Context;

    pub(super) fn sys_init() -> context::Context {
        println!("In game::sys_init()!");
        context::Context::init().unwrap()

    }

    pub(super) fn sys_update(gw: &mut types::GameWindow) {
        window::process_events(&mut gw.window, &mut gw.glfw, &mut gw.events);
        gw.window.swap_buffers();
    }

    pub(super) fn sys_cleanup() {
        println!("In game::sys_cleanup()!");
    }
}

pub fn run(init: fn() -> (), update: fn() -> (), cleanup: fn() -> ()) {
    let context = game::sys_init();
    let mut gamewindow = context.window;
    init();

    while !gamewindow.window.should_close() {
        game::sys_update(&mut gamewindow);
        update();
    }

    game::sys_cleanup();
    cleanup();
}
