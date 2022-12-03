mod primitives;
mod renderer_2d;
mod command;

use ultraviolet::Vec3;
pub(crate) use renderer_2d::Renderer2D;
pub(crate) use primitives::*;
pub(crate) use command::*;


#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub(crate) struct RenderContext {
    pub s: ShaderProgram,
    pub vao: VertexArray,
    pub vbo: VertexBuffer<Vertex>,
    pub ebo: IndexBuffer,
    pub cam: Camera2D,
    pub transform: Transform,
}

impl RenderContext {
    pub fn init() -> Self {
        let cam = Camera2D::new(-1.6, 1.6, -0.9, 0.9);
        let vertices = vec![
            Vertex { x: 0.0, y: 0.3, z: 0.0 },
            Vertex { x: -0.5, y: -0.5, z: 0.0 },
            Vertex { x: 0.5, y: -0.5, z: 0.0 },
        ];
        let colors = vec![
            Color { r: 1.0, g: 0.0, b: 0.0 },
            Color { r: 0.0, g: 1.0, b: 0.0 },
            Color { r: 0.0, g: 0.0, b: 1.0 },
        ];

        let indices = vec![0, 1, 2];

        unsafe {
            let s = ShaderProgram::new()
                .with_vert_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/main.vert.glsl"))
                .with_frag_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/main.frag.glsl"))
                .build();
            s.activate();

            let mut vao = VertexArray::new(gl::TRIANGLES);
            vao.bind();

            let vbo = VertexBuffer::new(vertices);
            let colors = VertexBuffer::new(colors);

            vao.index_count = indices.len() as u32;
            let ebo = IndexBuffer::new(indices);
            ebo.bind();
            
            vbo.bind();
            vao.link_attributes(&vbo, 0, 3, gl::FLOAT, (std::mem::size_of::<Vertex>()) as i32, 0 as *const _);
            colors.bind();
            vao.link_attributes(&colors, 1, 3, gl::FLOAT, (std::mem::size_of::<Color>()) as i32, 0 as *const _);

            vao.unbind();
            vbo.unbind();
            colors.unbind();
            ebo.unbind();

            let transform = Transform::new();

            RenderContext { s, vao, vbo, ebo, cam, transform }
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            command::set_clear_color(0.3, 0.3, 0.3, 1.);
            command::clear(gl::COLOR_BUFFER_BIT);

            self.cam.recalculate_matrices();

            let rotation = self.transform.get_rotation();
            let rotation = Vec3 { x: rotation.x + 0.005, y: rotation.y, z: rotation.z };
            self.transform.set_rotation(rotation);

            self.transform.set_position(Vec3 { x: 0.2, y: 0.2, z: 0. });
            self.transform.set_scale(Vec3 { x: 0.5, y: 0.5, z: 1. });

            let scene = Renderer2D::begin_scene(&self.cam);

            scene.submit(&self.vao, &self.s, &self.transform.get_matrix());

            scene.end();
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
