extern crate glfw;
use glfw::ffi::glfwGetTime;

static mut LAST_TIMESTEP: f64 = 0.;
static mut CURRENT_TIMESTEP: f64 = 0.;

pub struct Time {}

impl Time {
    pub fn now() -> f64 {
        unsafe {
            glfwGetTime()
        }
    }

    pub fn update() {
        unsafe {
            LAST_TIMESTEP = CURRENT_TIMESTEP;
            CURRENT_TIMESTEP = glfwGetTime();
        }
    }

    pub fn delta() -> f64 {
        unsafe { CURRENT_TIMESTEP - LAST_TIMESTEP }
    }
}
