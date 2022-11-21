extern crate gl;
use super::primitives::VertexArray;
use super::primitives::ShaderProgram;

pub(crate) struct Renderer2D {}

impl Renderer2D {
    pub unsafe fn draw_quad(vao: &VertexArray, shader: &ShaderProgram) {
        shader.activate();
        vao.bind();
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        vao.unbind();
    }

    pub unsafe fn draw(vao: &VertexArray, shader: &ShaderProgram) {
        shader.activate();
        vao.bind();
        gl::DrawElements(gl::TRIANGLES, vao.index_count as i32, gl::UNSIGNED_INT, 0 as *const _);
        vao.unbind();
        shader.deactivate();
    }

    pub unsafe fn draw_textured_quad(vao: &VertexArray) {

    }

    pub unsafe fn draw_text(vao: &VertexArray) {

    }
}
