extern crate glfw;

use super::asynchronous;
use crate::window;
use super::config;
use super::resources::ConfigContext;
use super::project;

pub(crate) struct Context {
    pub config_context: ConfigContext,
    pub window_context: window::WindowContext,
    pub async_context: asynchronous::AsyncContext,
    pub project_context: project::ProjectContext,
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
        let project_context = project::init(&String::from("/home/anthony/.alkahest/projects/main/"))?;

        return Ok(Self { window_context, async_context, config_context, project_context });
    }
}
