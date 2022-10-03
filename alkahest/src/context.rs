extern crate glfw;

use super::*;

pub(crate) struct Context {
    pub window: types::GameWindow,
}

impl Context {
    pub(crate) fn init() -> Result<Self, &'static str> {
        // initialize window system
        let window = window::init()?;

        return Ok(Self { window });
    }
}
