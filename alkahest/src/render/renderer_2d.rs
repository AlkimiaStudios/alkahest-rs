extern crate gl;
use super::primitives::VertexArray;
use super::primitives::ShaderProgram;
use super::primitives::Camera;
use super::command::*;
use crate::trace;
use ultraviolet::Mat4;

pub(crate) struct Renderer2D<'a> {
    camera: &'a dyn Camera,
}

impl<'a> Renderer2D<'a> {
    pub fn begin_scene(camera: &'a impl Camera) -> Self {
        Renderer2D {
            camera
        }
    }

    pub unsafe fn submit(&self, vao: &VertexArray, shader: &ShaderProgram, transform: &Mat4) {
        shader.activate();
        shader.set_uniform_mat4("projViewMat", self.camera.get_projection_view_matrix());
        shader.set_uniform_mat4("modelMat", transform);

        draw(vao);

        shader.deactivate();
    }

    pub unsafe fn end(&self) {
    }
}
