extern crate gl;

pub(crate) unsafe fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    gl::ClearColor(r, g, b, a);
}

pub(crate) unsafe fn clear() {
    gl::Clear(gl::COLOR_BUFFER_BIT);
}
