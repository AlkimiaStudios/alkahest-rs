extern crate glfw;

use super::types::GameWindow;
use glfw::Context;
use std::sync::mpsc::Receiver;

static mut GLFW_INIT: bool = false;

pub(crate) fn init() -> Result<GameWindow, &'static str> {
    unsafe {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        GLFW_INIT = true;

        // Create window with OpenGL context
        let (mut window, events) = glfw.create_window(300, 300, "Alkahest", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        // Make the window context current
        window.make_current();
        window.set_key_polling(true);

        // Poll for events
        glfw.poll_events();
        
        return Ok(GameWindow { glfw, window, events });
    }
}

pub(crate) fn process_events(window: &mut glfw::Window, glfw: &mut glfw::Glfw, events: &mut Receiver<(f64, glfw::WindowEvent)>) {
    glfw.poll_events();

    for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Close => {
                window.set_should_close(true);
            },
            _ => {},
        }
    }
}
