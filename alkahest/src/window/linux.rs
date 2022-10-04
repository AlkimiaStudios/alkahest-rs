extern crate glfw;

use super::super::util::types::WindowContext;
use glfw::Context;

static mut GLFW_INIT: bool = false;

pub(super) fn init(width: u32, height: u32, name: &str) -> Result<WindowContext, &'static str> {
    unsafe {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        GLFW_INIT = true;

        // Add X11 class + instance names
        glfw.window_hint(glfw::WindowHint::X11ClassName(Some(String::from("alkahest"))));
        glfw.window_hint(glfw::WindowHint::X11InstanceName(Some(String::from("alkahest"))));

        // Create window with OpenGL context
        let (mut window, events) = glfw.create_window(width, height, name, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window!");

        // Make the window context current
        window.make_current();
        window.set_key_polling(true);

        // Poll for events
        glfw.poll_events();
        
        Ok(WindowContext { glfw, window, events })
    }
}

pub(super) fn process_events(window_context: &mut WindowContext) {
    window_context.glfw.poll_events();

    for (_, event) in glfw::flush_messages(&window_context.events) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Close => {
                window_context.window.set_should_close(true);
            },
            _ => {},
        }
    }
}
