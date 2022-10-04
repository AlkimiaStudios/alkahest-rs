extern crate glfw;

use super::*;

pub(crate) struct Context {
    pub window_context: types::WindowContext,
}

impl Context {
    pub(crate) fn init() -> Result<Self, &'static str> {
        // initialize window system
        let window_context = super::super::window::init()?;

        return Ok(Self { window_context });
    }
}
