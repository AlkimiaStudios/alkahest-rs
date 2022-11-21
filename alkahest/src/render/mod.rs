mod primitives;
mod renderer_2d;

pub(crate) use primitives::VertexArray;
pub(crate) use primitives::VertexBuffer;
pub(crate) use primitives::IndexBuffer;
pub(crate) use primitives::ShaderProgram;
pub(crate) use renderer_2d::Renderer2D;

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
}

impl RenderContext {
    pub fn init() -> Self {
        let vertices = vec![
            Vertex { x: 0.0, y: 0.5, z: 0.0 },
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

            let mut vao = VertexArray::new();
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


            RenderContext { s, vao, vbo, ebo }
        }
    }

    pub fn draw(&self) {
        unsafe {
            Renderer2D::draw(&self.vao, &self.s);
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
