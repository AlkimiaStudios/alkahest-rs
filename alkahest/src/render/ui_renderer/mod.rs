use crate::{error, debug};
use crate::render::ui::*;
use crate::render::primitives::*;
use crate::render::command::*;
use crate::util::resources::TextureData;
use ultraviolet::{Vec2, Vec4, Mat4};

#[derive(Debug)]
struct UISceneData {
    pub camera_matrix: Mat4,
}

static mut SCENE_DATA: Option<UISceneData> = None;

#[derive(Debug)]
struct UIRendererContext {
    pub vao: VertexArray,
    pub positions: VertexBuffer<Vec4>,
    pub tex_coords: VertexBuffer<Vec2>,
    pub colors: VertexBuffer<Vec4>,
    pub indices: IndexBuffer,
    pub shader: ShaderProgram,
    pub default_panel_texture: Texture,
}

static mut RENDER_CONTEXT: Option<UIRendererContext> = None;

const MAX_QUADS: usize = 10000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;

const DEFAULT_TEX_BYTES: [u8; 4] = [255,255,255,255];

pub(crate) struct UIRenderer {}

impl UIRenderer {
    pub fn init() {
        unsafe {
            let vao = VertexArray::new(gl::TRIANGLES);
            vao.bind();

            let positions = VertexBuffer::<Vec4>::dynamic_new(MAX_VERTICES);
            let tex_coords = VertexBuffer::<Vec2>::dynamic_new(MAX_VERTICES);
            let colors = VertexBuffer::<Vec4>::dynamic_new(MAX_VERTICES);

            let indices = IndexBuffer::dynamic_new(MAX_INDICES);
            indices.bind();

            positions.bind();
            vao.link_attributes(&positions, 0, 4, gl::FLOAT, (std::mem::size_of::<Vec4>()) as i32, 0 as *const _);
            tex_coords.bind();
            vao.link_attributes(&tex_coords, 1, 2, gl::FLOAT, (std::mem::size_of::<Vec2>()) as i32, 0 as *const _);
            colors.bind();
            vao.link_attributes(&colors, 2, 4, gl::FLOAT, (std::mem::size_of::<Vec4>()) as i32, 0 as *const _);

            vao.unbind();
            colors.unbind();
            indices.unbind();

            let shader = ShaderProgram::new()
                .with_vert_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/ui.vert.glsl"))
                .with_frag_shader(String::from("/home/anthony/.alkahest/projects/main/shaders/ui.frag.glsl"))
                .build();

            let default_panel_texture = Texture::new_from_data(TextureData { bytes: DEFAULT_TEX_BYTES.to_vec(), width: 1, height: 1 });

            RENDER_CONTEXT = Some(UIRendererContext {
                vao,
                positions,
                tex_coords,
                colors,
                indices,
                shader,
                default_panel_texture,
            });
        }
    }

    pub fn begin_scene(camera: &impl Camera) {
        unsafe {
            if let Some(context) = &RENDER_CONTEXT {
                let data = UISceneData {
                    camera_matrix: camera.get_projection_view_matrix().clone(),
                };

                context.shader.activate();
                context.shader.set_uniform_mat4("u_projViewMat", &data.camera_matrix);

                SCENE_DATA = Some(data);
            }
            else {
                error!("Renderer2D has not been initialized!");
            }
        }
    }

    pub fn end_scene() {
        unsafe {
            SCENE_DATA = None;
        }
    }

    pub fn cleanup() {
        unsafe {
            RENDER_CONTEXT = None;
        }
    }

    fn clear_buffers() {
        todo!();
    }

    pub fn draw_tree(root: &UIRoot) {
        todo!();
    }

    pub fn draw_panel(panel: &UIPanel) {
        unsafe {
            if let Some(context) = &mut RENDER_CONTEXT {
                let batch = panel.get_batch();
                context.positions.set_data(&batch.position_data, batch.vertex_count, 0);
                context.tex_coords.set_data(&batch.tex_coord_data, batch.vertex_count, 0);
                context.colors.set_data(&batch.color_data, batch.vertex_count, 0);
                context.indices.set_data(&batch.index_data, batch.index_count, 0);
                context.vao.vertex_count = batch.vertex_count as u32;
                context.vao.index_count = batch.index_count as u32;

                context.shader.activate();

                let tex = batch.texture.unwrap_or(&context.default_panel_texture);
                tex.bind(0);
                context.shader.set_uniform_int("u_panelTexture", 0);

                draw(&context.vao);
                context.shader.deactivate();
            }
            else {
                error!("UIRenderer not initialized!");
            }
        }
    }
}
