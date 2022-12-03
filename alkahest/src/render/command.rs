extern crate gl;

use crate::render::VertexArray;
use crate::{trace, error};

pub(crate) unsafe fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    gl::ClearColor(r, g, b, a);
}

pub(crate) unsafe fn clear(mask: gl::types::GLbitfield) {
    gl::Clear(mask);
}

pub(crate) unsafe fn draw(vao: &VertexArray) {
    if vao.index_count > 0 {
        draw_indexed(vao);
    }
    else if vao.vertex_count > 0 {
        draw_arrays(vao);
    }
    else {
        error!("render::command::draw() called with an invalid vertex array!");
    }
}

unsafe fn draw_indexed(vao: &VertexArray) {
    vao.bind();
    gl::DrawElements(vao.draw_method, vao.index_count as i32, gl::UNSIGNED_INT, 0 as *const _);
    vao.unbind();
}

unsafe fn draw_arrays(vao: &VertexArray) {
    vao.bind();
    gl::DrawArrays(vao.draw_method, 0, vao.vertex_count as i32);
    vao.unbind();
}
