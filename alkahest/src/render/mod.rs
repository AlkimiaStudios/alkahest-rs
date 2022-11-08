mod vertex_array;
mod vertex_buffer;
mod index_buffer;

use vertex_array::VertexArray;
use vertex_buffer::VertexBuffer;
use index_buffer::IndexBuffer;
use crate::trace;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub(crate) struct RenderContext {
    pub vao: VertexArray,
    pub vbo: VertexBuffer<Vertex>,
    pub ebo: IndexBuffer,
}

impl RenderContext {
    pub fn init() -> Self {
        let vertices = vec![
            Vertex { x: 0.0, y: 0.5, z: 0.0 },
            Vertex { x: -0.5, y: -0.5, z: 0.0 },
            Vertex { x: 0.5, y: -0.5, z: 0.0 },
        ];

        let indices = vec![0, 1, 2];

        unsafe {
            let vao = VertexArray::new();
            vao.bind();

            let vbo = VertexBuffer::new(vertices);
            vbo.bind();
            let ebo = IndexBuffer::new(indices);
            ebo.bind();

            vao.link_attributes(&vbo, 0, 3, gl::FLOAT, (std::mem::size_of::<Vertex>()) as i32, 0 as *const _);

            vao.unbind();
            vbo.unbind();
            ebo.unbind();

            RenderContext { vao, vbo, ebo }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, 0 as *const _);
            self.vao.unbind();
        }        
    }

    pub fn cleanup(&self) {
        unsafe {
            self.vao.destroy();
            self.vbo.destroy();
            self.ebo.destroy();
        }
    }
}
