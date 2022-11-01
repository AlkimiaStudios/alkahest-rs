extern crate glfw;

use super::asynchronous;
use crate::window;

pub(crate) struct Context {
    pub window_context: window::WindowContext,
    pub async_context: asynchronous::AsyncContext,
}

impl Context {
    pub(crate) fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // initialize window system
        let window_context = window::init(1920, 1080, "Alkahest")?;
        let async_context = asynchronous::init();

        return Ok(Self { window_context, async_context });
    }
}
