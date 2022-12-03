#![warn(missing_docs)]
//! The simple game engine written in Rust and OpenGL.
//!
//! This is the core library of the Alakhest Game engine. I
//! should probably add some more docs here eventually...

/// Contains functions used internally for managing the engine runtime.
pub(crate) mod game;
pub use game::Time;
/// Render stuff
pub mod render;
/// Contains utility functions and structs for use inside and outside the engine.
pub mod util;
/// Contains functions used internally for creating a window and rendering context.
pub(crate) mod window;

/// The Application trait enables Alkahest users to define game-side logic
/// for starting, updating, and cleaning up the game state.
///
/// When creating an application with Alkahest, implement this trait on the
/// struct that will hold your game logic. An instance of that struct will
/// be passed to the `run` method below, allowing the engine and game to start.
pub trait Application {
    /// The function used to initialize the game state.
    fn init(&mut self);
    
    /// The function used to update the game state.
    fn update(&mut self, delta: f64);

    /// The function used to clean up the game state.
    fn cleanup(&mut self);
}

/// The primary entrypoint for the Alkahest engine.
pub fn run<T>(app: &mut T) where T: Application {
    error!("This is an error message!");
    warn!("This is a warning message.");
    info!("This is an info message.");
    debug!("This is a debug message.");
    trace!("This is a trace message...");
    let mut engine_context = game::sys_init();
    app.init();

    while !engine_context.window_context.window.should_close() {
        let delta = Time::delta();
        game::sys_update(delta, &mut engine_context.window_context);

        engine_context.render_context.draw();

        app.update(delta);

        Time::update();
    }

    game::sys_cleanup(engine_context);
    app.cleanup();
}
