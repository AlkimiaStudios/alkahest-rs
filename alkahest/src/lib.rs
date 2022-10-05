#![warn(missing_docs)]
//! The simple game engine written in Rust and OpenGL.
//!
//! This is the core library of the Alakhest Game engine. I
//! should probably add some more docs here eventually...

/// Contains functions used internally for managing the engine runtime.
pub(crate) mod game;
/// Contains utility functions and structs for use inside and outside the engine.
pub mod util;
/// Contains functions used internally for creating a window and rendering context.
pub(crate) mod window;

/// The primary entrypoint for the Alkahest engine.
///
/// When creating an application with Alkahest, import this function
/// and supply it with the three primary methods of the application:
/// `init()`, `update()`, and `cleanup()`.
///
/// * init: fn() -> () - Function used to initialize application state.
/// * update: fn() -> () - Function used to update application state.
/// * cleanup: fn() -> () - Function used to clean up application state.
///
/// ```
/// fn init() {}
/// fn update() {}
/// fn cleanup() {}
///
/// alkahest::run(init, update, cleanup);
/// ```
pub fn run(init: fn() -> (), update: fn() -> (), cleanup: fn() -> ()) {
    error!("This is an error message!");
    warn!("This is a warning message.");
    info!("This is an info message.");
    debug!("This is a debug message.");
    trace!("This is a trace message...");
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
