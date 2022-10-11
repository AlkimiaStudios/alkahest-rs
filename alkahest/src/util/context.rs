extern crate glfw;

use super::{types, asynchronous};

pub(crate) struct Context {
    pub window_context: types::WindowContext,
    pub async_context: asynchronous::AsyncContext,
}

impl Context {
    pub(crate) fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // initialize window system
        let window_context = super::super::window::init(1920, 1080, "Alkahest")?;
        let async_context = super::asynchronous::init();

        return Ok(Self { window_context, async_context });
    }
}
