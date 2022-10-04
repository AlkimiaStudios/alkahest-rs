extern crate glfw;
use std::sync::mpsc::Receiver;
use glfw::{Glfw, Window, WindowEvent};

pub(crate) struct WindowContext {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}
