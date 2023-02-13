use ultraviolet::{Vec2, Vec4};
use super::Texture;

// TODO: Convert Batch to a trait?
// Potentially useful to be able to get the data using a method
// and also add a render method to the trait, both can be implemented
// using #[derive] 

#[derive(Debug)]
pub struct Batch<'a> {
    pub position_data: &'a Vec<Vec4>,
    pub tex_coord_data: &'a Vec<Vec2>,
    pub color_data: &'a Vec<Vec4>,
    pub index_data: &'a Vec<u32>,
    pub vertex_count: usize,
    pub index_count: usize,
    pub texture: Option<&'a Texture>,
}
