extern crate gl;
use super::primitives::VertexArray;
use super::primitives::VertexBuffer;
use super::primitives::IndexBuffer;
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
    pub vao: VertexArray,
    pub positions: VertexBuffer<Vec3>,
    pub tex_coords: VertexBuffer<Vec2>,
    pub tex_ids: VertexBuffer<u32>,
    pub colors: VertexBuffer<Vec4>,
    pub ebo: IndexBuffer,
    pub shader: ShaderProgram,
    pub default_texture: Texture,

    pub position_data: Vec<Vec3>,
    pub tex_coord_data: Vec<Vec2>,
    pub tex_id_data: Vec<u32>,
    pub color_data: Vec<Vec4>,
    pub indices: Vec<u32>,
    pub textures: [Option<Texture>; MAX_TEXTURES],

    pub vertex_count: usize,
    pub index_count: usize,
    pub texture_count: usize,
}

static mut RENDER_CONTEXT: Option<Renderer2DContext> = None;

const QUAD_VERTICES: [Vec4; 4] = [
    Vec4::new(-0.5,  0.5, 0., 0.), // top left
    Vec4::new( 0.5,  0.5, 0., 0.), // top right
    Vec4::new(-0.5, -0.5, 0., 0.), // bottom left
    Vec4::new( 0.5, -0.5, 0., 0.), // bottom right
];

// const QUAD_TEX_COORDS: [Vec2; 4] = [
    // Vec2::new(0., 0.), // top left
    // Vec2::new(1., 0.), // top right
    // Vec2::new(0., 1.), // bottom left
    // Vec2::new(1., 1.), // bottom right
// ];

const MAX_QUADS: usize = 10000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_TEXTURES: usize = 32;

const DEFAULT_TEX_BYTES: [u8; 4] = [255,255,255,255];

pub(crate) struct Renderer2D {}

impl Renderer2D {
    pub unsafe fn init() {
        let shader = ShaderProgram::new()
            .with_vert_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/quad.vert.glsl"))
            .with_frag_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/quad.frag.glsl"))
            .build();

        let vao = VertexArray::new(gl::TRIANGLES);
        vao.bind();

        let positions = VertexBuffer::<Vec3>::dynamic_new(MAX_VERTICES);
        let tex_coords = VertexBuffer::<Vec2>::dynamic_new(MAX_VERTICES);
        let tex_ids = VertexBuffer::<u32>::dynamic_new(MAX_VERTICES);
        let colors = VertexBuffer::<Vec4>::dynamic_new(MAX_VERTICES);

        let ebo = IndexBuffer::dynamic_new(MAX_INDICES);
        ebo.bind();

        positions.bind();
        vao.link_attributes(&positions, 0, 3, gl::FLOAT, (std::mem::size_of::<Vec3>()) as i32, 0 as *const _);
        tex_coords.bind();
        vao.link_attributes(&tex_coords, 1, 2, gl::FLOAT, (std::mem::size_of::<Vec2>()) as i32, 0 as *const _);
        tex_ids.bind();
        vao.link_attributes(&tex_ids, 2, 1, gl::UNSIGNED_INT, (std::mem::size_of::<u32>()) as i32, 0 as *const _);
        colors.bind();
        vao.link_attributes(&colors, 3, 4, gl::FLOAT, (std::mem::size_of::<Vec4>()) as i32, 0 as *const _);

        vao.unbind();
        colors.unbind();
        ebo.unbind();

        let position_data: Vec<Vec3> = vec![Vec3::zero(); MAX_VERTICES];
        let tex_coord_data: Vec<Vec2> = vec![Vec2::zero(); MAX_VERTICES];
        let tex_id_data: Vec<u32> = vec![0; MAX_VERTICES];
        let color_data: Vec<Vec4> = vec![Vec4::zero(); MAX_VERTICES];
        let indices: Vec<u32> = vec![0; MAX_INDICES];

        let default_texture = Texture::new_from_data(TextureData { bytes: DEFAULT_TEX_BYTES.to_vec(), width: 1, height: 1 });
        let mut textures: [Option<Texture>; MAX_TEXTURES] = [None; MAX_TEXTURES];
        textures[0] = Some(default_texture);

        RENDER_CONTEXT = Some(Renderer2DContext { vao, positions, tex_coords, tex_ids, colors, ebo, shader, default_texture, position_data, tex_coord_data, tex_id_data, color_data, indices, textures, vertex_count: 0, index_count: 0, texture_count: 1 });
    }

    pub unsafe fn begin_scene(camera: &impl Camera) {
        if let Some(context) = &RENDER_CONTEXT {
            let data = SceneData2D {
                camera_matrix: camera.get_projection_view_matrix().clone(),
            };

            context.shader.activate();
            context.shader.set_uniform_mat4("projViewMat", &data.camera_matrix);

            // let mut texture_slots: Vec<u32> = vec![0; MAX_TEXTURES];
            // for i in 0..MAX_TEXTURES {
                // texture_slots[i] = i as u32;    
            // }

            // context.shader.set_uniform_int_arr("textureSlots", &texture_slots, MAX_TEXTURES);

            SCENE_DATA = Some(data);
        }
        else {
            error!("Renderer2D has not been initialized!");
        }
    }

    pub unsafe fn draw_quad(position: Vec3, size: Vec2, rotation: f32, color: Vec4, texture: Option<&Texture>) {
        if let Some(context) = &mut RENDER_CONTEXT {
            if let Some(_scene_data) = &SCENE_DATA {
                // calculate transform
                let transform: Mat4 = Mat4::identity().translated(&position);
                let transform = transform * Mat4::from_rotation_z(rotation);
                let transform = transform * Mat4::from_nonuniform_scale(Vec3::new(size.x, size.y, 1.));
                
                // apply transform to position data
                // and set position data for 4 vertices starting at vertex_count
                context.position_data[context.vertex_count]     = (transform * Vec4::new(-0.5,  0.5, 0., 1.)).xyz();
                context.position_data[context.vertex_count + 1] = (transform * Vec4::new( 0.5,  0.5, 0., 1.)).xyz();
                context.position_data[context.vertex_count + 2] = (transform * Vec4::new(-0.5, -0.5, 0., 1.)).xyz();
                context.position_data[context.vertex_count + 3] = (transform * Vec4::new( 0.5, -0.5, 0., 1.)).xyz();

                // set tex_coord data
                context.tex_coord_data[context.vertex_count]     = Vec2::new(0., 0.);
                context.tex_coord_data[context.vertex_count + 1] = Vec2::new(1., 0.);
                context.tex_coord_data[context.vertex_count + 2] = Vec2::new(0., 1.);
                context.tex_coord_data[context.vertex_count + 3] = Vec2::new(1., 1.);

                // set tex_id
                let mut tex_id: u32 = 0;
                if let Some(tex) = texture {
                    // context.shader.activate();
                    // tex.bind(1);
                    // context.shader.set_uniform_int("quadTex", 1);
                    for i in 1..context.texture_count {
                        if let Some(existing_tex) = context.textures[i] {
                            if existing_tex.id == tex.id {
                                tex_id = i as u32;
                            }
                        }
                    }
                    // No matching textures
                    if tex_id == 0 {
                        context.textures[context.texture_count] = Some(tex.clone());
                        tex_id = context.texture_count as u32;
                        context.texture_count = context.texture_count + 1;
                        trace!("TextureCount: {}", context.texture_count);
                    }
                }
                trace!("TexID: {}", tex_id);

                context.tex_id_data[context.vertex_count] = tex_id;
                context.tex_id_data[context.vertex_count + 1] = tex_id;
                context.tex_id_data[context.vertex_count + 2] = tex_id;
                context.tex_id_data[context.vertex_count + 3] = tex_id;

                // set color data
                context.color_data[context.vertex_count] = color;
                context.color_data[context.vertex_count + 1] = color;
                context.color_data[context.vertex_count + 2] = color;
                context.color_data[context.vertex_count + 3] = color;

                // set index data for 6 indices starting at index_count
                context.indices[context.index_count] = context.vertex_count as u32;
                context.indices[context.index_count + 1] = context.vertex_count as u32 + 1;
                context.indices[context.index_count + 2] = context.vertex_count as u32 + 2;
                context.indices[context.index_count + 3] = context.vertex_count as u32 + 1;
                context.indices[context.index_count + 4] = context.vertex_count as u32 + 2;
                context.indices[context.index_count + 5] = context.vertex_count as u32 + 3;

                // Increment vertex and index count
                context.vertex_count = context.vertex_count + 4;
                context.index_count = context.index_count + 6;
                
                if context.vertex_count >= MAX_VERTICES {
                    Renderer2D::flush();
                    Renderer2D::execute();
                }
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

        Renderer2D::flush();
        Renderer2D::execute();
    }

    unsafe fn flush() {
        if let Some(context) = &mut RENDER_CONTEXT {
            context.positions.set_data(&context.position_data, context.vertex_count);
            context.tex_coords.set_data(&context.tex_coord_data, context.vertex_count);
            context.tex_ids.set_data(&context.tex_id_data, context.vertex_count);
            context.colors.set_data(&context.color_data, context.vertex_count);

            context.ebo.set_data(&context.indices, context.index_count);

            context.vao.vertex_count = context.vertex_count as u32;
            context.vao.index_count = context.index_count as u32;

            context.vertex_count = 0;
            context.index_count = 0;
        }                
    }

    unsafe fn execute() {
        if let Some(context) = &RENDER_CONTEXT {
            context.shader.activate();

            for i in 0..context.texture_count {
                if let Some(tex) = context.textures[i] {
                    tex.bind(i as u32);
                } else {
                    context.default_texture.bind(i as u32);
                }
            }
            trace!("Texture count: {}", context.texture_count);

            let mut texture_slots: Vec<u32> = vec![0; MAX_TEXTURES];
            for i in 0..MAX_TEXTURES {
                texture_slots[i] = i as u32;    
            }

            context.shader.set_uniform_int_arr("textureSlots", &texture_slots, MAX_TEXTURES);

            trace!("Sending batch of {} quads to GPU", context.vao.vertex_count / 4);
            draw(&context.vao);

            context.shader.deactivate();
        }
    }

    pub unsafe fn cleanup() {
        RENDER_CONTEXT = None;
    }
}
