pub(crate) mod util;
pub(crate) mod window;
mod game;

pub fn run(init: fn() -> (), update: fn() -> (), cleanup: fn() -> ()) {
    let context = game::sys_init();
    let mut window_context = context.window_context;
    init();

    while !window_context.window.should_close() {
        game::sys_update(&mut window_context);
        update();
    }

    game::sys_cleanup();
    cleanup();
}
