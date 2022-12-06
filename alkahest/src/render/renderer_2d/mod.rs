extern crate gl;
use super::primitives::VertexArray;
use super::primitives::VertexBuffer;
use super::primitives::ShaderProgram;
use super::primitives::Texture;
use super::primitives::Camera;
use super::command::*;
use crate::{trace, error, util::resources::TextureData};
use ultraviolet::{Mat4, Vec2, Vec3, Vec4};

#[derive(Debug)]
struct SceneData2D {
    pub camera_matrix: Mat4,
}

static mut SCENE_DATA: Option<SceneData2D> = None;

#[derive(Debug)]
struct Renderer2DContext {
    pub quad_vao: VertexArray,
    pub shader: ShaderProgram,
    pub default_texture: Texture,
}

static mut RENDER_CONTEXT: Option<Renderer2DContext> = None;

const QUAD_VERTICES: [Vec3; 4] = [
    Vec3::new(-0.5,  0.5, 0.), // top left
    Vec3::new( 0.5,  0.5, 0.), // top right
    Vec3::new(-0.5, -0.5, 0.), // bottom left
    Vec3::new( 0.5, -0.5, 0.), // bottom right
];

const QUAD_TEX_COORDS: [Vec2; 4] = [
    Vec2::new(0., 0.), // top left
    Vec2::new(1., 0.), // top right
    Vec2::new(0., 1.), // bottom left
    Vec2::new(1., 1.), // bottom right
];

const DEFAULT_TEX_BYTES: [u8; 4] = [255,255,255,255];

pub(crate) struct Renderer2D {}

impl Renderer2D {
    pub unsafe fn init() {
        let shader = ShaderProgram::new()
            .with_vert_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/quad.vert.glsl"))
            .with_frag_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/quad.frag.glsl"))
            .build();

        let mut vao = VertexArray::new(gl::TRIANGLE_STRIP);
        vao.bind();

        let vbo = VertexBuffer::new_from_arr(&QUAD_VERTICES);
        let tex = VertexBuffer::new_from_arr(&QUAD_TEX_COORDS);
        vao.vertex_count = 4;
        
        vbo.bind();
        vao.link_attributes(&vbo, 0, 3, gl::FLOAT, (std::mem::size_of::<Vec3>()) as i32, 0 as *const _);
        tex.bind();
        vao.link_attributes(&tex, 1, 2, gl::FLOAT, (std::mem::size_of::<Vec2>()) as i32, 0 as *const _);

        vao.unbind();
        vbo.unbind();
        tex.unbind();

        let default_texture = Texture::new_from_data(TextureData { bytes: DEFAULT_TEX_BYTES.to_vec(), width: 1, height: 1 }, 0 );

        RENDER_CONTEXT = Some(Renderer2DContext { quad_vao: vao, shader, default_texture });
    }

    pub unsafe fn begin_scene(camera: &impl Camera) {
        if let Some(context) = &RENDER_CONTEXT {
            let data = SceneData2D {
                camera_matrix: camera.get_projection_view_matrix().clone(),
            };

            context.shader.activate();
            context.shader.set_uniform_mat4("projViewMat", &data.camera_matrix);

            SCENE_DATA = Some(data);
        }
        else {
            error!("Renderer2D has not been initialized!");
        }
    }

    pub unsafe fn draw_quad(position: Vec3, size: Vec2, rotation: f32, color: Vec4, texture: Option<&Texture>) {
        if let Some(context) = &RENDER_CONTEXT {
            if let Some(_scene_data) = &SCENE_DATA {
                let shader = &context.shader;
                let tex = texture.unwrap_or(&context.default_texture);
                shader.activate();

                // Compute transform from pos/size/rotation
                let transform: Mat4 = Mat4::identity().translated(&position);
                let transform = transform * Mat4::from_rotation_z(rotation);
                let transform = transform * Mat4::from_nonuniform_scale(Vec3::new(size.x, size.y, 1.));
                shader.set_uniform_mat4("modelMat", &transform);

                shader.set_uniform_vec4("quadColor", &color);

                tex.bind();
                shader.set_uniform_int("quadTexture", tex.slot);

                draw(&context.quad_vao);

                tex.unbind();

                shader.deactivate();
            }
            else {
                error!("Renderer2D cannot render without a current scene!");
            }
        }
        else {
            error!("Renderer2D has not been initialized!");
        }
    }

    pub unsafe fn end_scene() {
        SCENE_DATA = None;
    }

    pub unsafe fn cleanup() {
        RENDER_CONTEXT = None;
    }
}
