extern crate gl;

use ultraviolet::{Vec2, Vec3, Vec4, Mat4};
use crate::render::{Batch, Texture};

use super::UIElement;

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

const MAX_QUADS: usize = 1000;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_INDICES: usize = MAX_QUADS * 6;

pub struct UIPanel<'a> {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Vec4,
    pub texture: Option<&'a Texture>,
    pub children: Vec<Box<dyn UIElement>>,
    position_data: Vec<Vec4>,
    tex_coord_data: Vec<Vec2>,
    color_data: Vec<Vec4>,
    index_data: Vec<u32>,
    vertex_count: usize,
    index_count: usize,
}

// TODO: Draw non-overlapping panels in same batch
// TODO: Set up batch to draw itself???
// TODO: System for updating batch data for children

impl<'a> UIPanel<'a> {
    pub fn new(position: Vec2, size: Vec2, rotation: f32, color: Vec4, texture: Option<&'a Texture>) -> Self {
        let mut position_data = Vec::<Vec4>::with_capacity(MAX_VERTICES);
        let mut tex_coord_data = Vec::<Vec2>::with_capacity(MAX_VERTICES);
        let mut color_data = Vec::<Vec4>::with_capacity(MAX_VERTICES);
        let mut index_data = Vec::<u32>::with_capacity(MAX_INDICES);
        let mut vertex_count: usize = 0;
        let mut index_count: usize = 0;

        let transform = Mat4::from_translation(Vec3::new(position.x, position.y, 0.));
        let transform = transform * Mat4::from_rotation_z(rotation);
        let transform = transform * Mat4::from_nonuniform_scale(Vec3::new(size.x, size.y, 1.));
        
        let positions: Vec<Vec4> = QUAD_VERTICES.iter().map(|vertex| transform * vertex.to_owned()).collect();
        position_data.extend_from_slice(positions.as_slice());
        tex_coord_data.extend_from_slice(&QUAD_TEX_COORDS);
        color_data.extend_from_slice(&[color; 4]);
        index_data.extend_from_slice(&QUAD_INDICES);
        vertex_count += 4;
        index_count += 6;

        return UIPanel {
            position,
            size,
            rotation,
            color,
            texture,
            children: Vec::new(),
            position_data,
            tex_coord_data,
            color_data,
            index_data,
            vertex_count,
            index_count,
        }
    }

    pub fn add_child(&mut self, child: Box<dyn UIElement>) -> usize {
        let transform = self.get_transform();
        let batch = child.get_data_for_buffer();
        let positions: Vec<Vec4> = batch.position_data.iter().map(|vertex| transform * vertex.to_owned()).collect();
        let indices: Vec<u32> = batch.index_data.iter().map(|index| index + self.vertex_count as u32).collect();

        self.position_data.extend_from_slice(positions.as_slice());
        self.tex_coord_data.extend_from_slice(&batch.tex_coord_data);
        self.color_data.extend_from_slice(&batch.color_data);
        self.index_data.extend_from_slice(indices.as_slice());
        self.vertex_count += batch.vertex_count;
        self.index_count += batch.index_count;

        self.children.push(child);
        return self.children.len() - 1;
    }

    pub fn remove_child(&mut self, index: usize) {
        // When we remove a child, we need to reprocess the buffer to build
        // it back up from scratch
        // TODO: Optimize this by only removing the data for the child
        self.children.remove(index);

        self.clear_buffer();
        let transform = self.get_transform();
        self.gather_panel_buffer_data(transform);
        self.gather_child_buffer_data(transform);
    }

    fn clear_buffer(&mut self) {
        self.position_data.clear();
        self.tex_coord_data.clear();
        self.color_data.clear();
        self.index_data.clear();
        self.vertex_count = 0;
        self.index_count = 0;
    }

    fn get_transform(&self) -> Mat4 {
        let transform = Mat4::from_translation(Vec3::new(self.position.x, self.position.y, 0.));
        let transform = transform * Mat4::from_rotation_z(self.rotation);
        let transform = transform * Mat4::from_nonuniform_scale(Vec3::new(self.size.x, self.size.y, 1.));
        return transform;
    }

    fn gather_panel_buffer_data(&mut self, transform: Mat4) {
        let positions: Vec<Vec4> = QUAD_VERTICES.iter().map(|vertex| transform * vertex.to_owned()).collect();
        self.position_data.extend_from_slice(positions.as_slice());
        self.tex_coord_data.extend_from_slice(&QUAD_TEX_COORDS);
        self.color_data.extend_from_slice(&[self.color; 4]);
        self.index_data.extend_from_slice(&QUAD_INDICES);
        self.vertex_count += 4;
        self.index_count += 6;
    }

    fn gather_child_buffer_data(&mut self, transform: Mat4) {
        self.children.iter().map(|child| child.get_data_for_buffer())
            .for_each(|batch| {
                let positions: Vec<Vec4> = batch.position_data.iter().map(|vertex| transform * vertex.to_owned()).collect();
                self.position_data.extend_from_slice(positions.as_slice());
                self.tex_coord_data.extend_from_slice(&batch.tex_coord_data);
                self.color_data.extend_from_slice(&batch.color_data);
                self.index_data.extend_from_slice(&batch.index_data);
                self.vertex_count += batch.vertex_count;
                self.index_count += batch.index_count;
            });
    }

    pub fn get_batch(&self) -> Batch {
        Batch {
            position_data: &self.position_data,
            tex_coord_data: &self.tex_coord_data,
            color_data: &self.color_data,
            index_data: &self.index_data,
            vertex_count: self.vertex_count,
            index_count: self.index_count,
            texture: self.texture,
        }
    }
}
