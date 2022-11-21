extern crate glfw;

use super::asynchronous;
use crate::window;
use super::config;
use super::resources::ConfigContext;
use crate::render::RenderContext;

pub(crate) struct Context {
    pub config_context: ConfigContext,
    pub window_context: window::WindowContext,
    pub async_context:  asynchronous::AsyncContext,
    pub render_context: RenderContext,
}

impl Context {
    pub(crate) fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let config_context = config::init();
        let window_context = window::init(
            config_context.window.width,
            config_context.window.height,
            &config_context.window.title,
            &config_context.window.hint)?;
        let async_context = asynchronous::init();

        let render_context = RenderContext::init();

        return Ok(Self { window_context, async_context, config_context, render_context });
    }
}
