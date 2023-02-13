mod root;
pub use root::UIRoot;
mod panel;
pub use panel::UIPanel;

use super::Batch;
use ultraviolet::{Vec2, Vec3, Vec4, Mat4};

pub trait UIElement {
    fn get_data_for_buffer(&self) -> Batch;
}

const QUAD_VERTICES: [Vec4; 4] = [
    Vec4::new(-0.5,  0.5, 0., 1.), // top left
    Vec4::new( 0.5,  0.5, 0., 1.), // top right
    Vec4::new(-0.5, -0.5, 0., 1.), // bottom left
    Vec4::new( 0.5, -0.5, 0., 1.), // bottom right
];

const QUAD_TEX_COORDS: [Vec2; 4] = [
    Vec2::new(0., 0.), // top left
    Vec2::new(1., 0.), // top right
    Vec2::new(0., 1.), // bottom left
    Vec2::new(1., 1.), // bottom right
];

const QUAD_INDICES: [u32; 6] = [
    0, 1, 2, // first triangle
    1, 2, 3, // second triangle
];

pub struct BaseChild {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    positions: Vec<Vec4>,
    tex_coords: Vec<Vec2>,
    colors: Vec<Vec4>,
    indices: Vec<u32>,
    transform: Mat4,
}

impl BaseChild {
    pub fn new(position: Vec2, size: Vec2, rotation: f32) -> Self {
        let transform = Mat4::from_translation(Vec3::new(position.x, position.y, 0.));
        let transform = transform * Mat4::from_rotation_z(rotation);
        let transform = transform * Mat4::from_nonuniform_scale(Vec3::new(size.x, size.y, 1.));

        let positions: Vec<Vec4> = QUAD_VERTICES.iter().map(|vertex| transform * vertex.to_owned()).collect();

        BaseChild {
            position,
            size,
            rotation,
            positions,
            tex_coords: QUAD_TEX_COORDS.to_vec(),
            colors: vec![Vec4::new(1., 1., 1., 1.); 4],
            indices: QUAD_INDICES.to_vec(),
            transform,
        }
    }
}

impl UIElement for BaseChild {
    fn get_data_for_buffer(&self) -> Batch {
        Batch {
            position_data: &self.positions,
            tex_coord_data: &self.tex_coords,
            color_data: &self.colors,
            index_data: &self.indices,
            vertex_count: 4,
            index_count: 6,
            texture: None,
        }
    }
}
