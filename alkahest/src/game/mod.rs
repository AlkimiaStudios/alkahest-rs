use super::*;
use glfw::Context;

/// Initializes the engine systems and stores engine context.
///
/// This method is called at the beginning of the `run()` function
/// to initialize various engine subsystems. It also returns a
/// Context instance to be used during the life of the engine for
/// various processes/systems.
#[doc(hidden)]
pub(super) fn sys_init() -> util::context::Context {
    trace!("Initializing engine context");
    let context = match util::context::Context::init() {
        Err(e) => panic!("Unable to initialize engine context! Error: {:?}", e),
        Ok(context) => context,
    };

    return context;
}

/// Updates the current engine state on each game loop cycle.
///
/// This method is called each time the game loop cycles. It is
/// passed context objects to provide necessary state to each engine
/// subsystem.
#[doc(hidden)]
pub(super) fn sys_update(window_context: &mut util::types::WindowContext) {
    window::process_events(window_context);
    window_context.window.swap_buffers();
}

/// Cleans up any systems in use by then engine, including context.
///
/// This method is called at the end of the engine lifetime, in order
/// to clean up any resources that were provisioned during runtime, if
/// necessary.
#[doc(hidden)]
pub(super) fn sys_cleanup(context: util::context::Context) {
    trace!("In game::sys_cleanup()!");
    util::asynchronous::cleanup(&context.async_context);
}
