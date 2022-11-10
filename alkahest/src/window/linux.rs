extern crate glfw;
extern crate gl;

use super::super::util::input;
use crate::trace;
use glfw::{Context, Glfw, Window, WindowEvent};
use std::sync::mpsc::Receiver;

pub(crate) struct WindowContext {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

static mut GLFW_INIT: bool = false;

pub(super) fn init(width: u32, height: u32, name: &str, hint: &str) -> Result<WindowContext, &'static str> {
    unsafe {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
        if let Err(_) = glfw {
            return Err("Failed to initialize GLFW!");
        }

        let mut glfw = glfw.unwrap_unchecked();
        GLFW_INIT = true;

        // Add X11 class + instance names
        glfw.window_hint(glfw::WindowHint::X11ClassName(Some(String::from(hint))));
        glfw.window_hint(glfw::WindowHint::X11InstanceName(Some(String::from(hint))));

        // Create window with OpenGL context
        let window_result = glfw.create_window(width, height, name, glfw::WindowMode::Windowed);
        if let None = window_result {
            return Err("Failed to create window with GLFW!");
        }
        let (mut window, events) = window_result.unwrap_unchecked();

        // Make the window context current
        window.make_current();
        window.set_all_polling(true);

        gl::load_with(|s| window.get_proc_address(s) as *const _);

        // Poll for events
        glfw.poll_events();
        
        Ok(WindowContext { glfw, window, events })
    }
}

pub(super) fn process_events(window_context: &mut WindowContext) {
    window_context.glfw.poll_events();

    for (_, event) in glfw::flush_messages(&window_context.events) {
        match event {
            glfw::WindowEvent::Key(key, _scancode, action, _modifiers) => {
                trace!("KeyEvent! Key: {:?}, Action: {:?}", key, action);
                let state = action == glfw::Action::Press || action == glfw::Action::Repeat;
                input::set_key(key as u32, state);
            },
            glfw::WindowEvent::MouseButton(button, action, _modifiers) => {
                trace!("MouseButtonEvent! Button: {:?}, Action: {:?}", button, action);
                input::set_mouse_button(button as u32, action == glfw::Action::Press);
            },
            glfw::WindowEvent::CursorPos(x, y) => {
                trace!("CursorPosEvent! X: {:?}, Y: {:?}", x, y);
                input::set_mouse_pos(x as f32, y as f32);
            },
            glfw::WindowEvent::Scroll(x, y) => {
                trace!("ScrollEvent! X: {:?}, Y: {:?}", x, y);
                input::set_mouse_scroll(x as f32, y as f32);
            },
            glfw::WindowEvent::Close => {
                window_context.window.set_should_close(true);
            },
            _ => {},
        }
    }
}
