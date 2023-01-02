mod primitives;
mod renderer_2d;
mod command;

use ultraviolet::{Vec2,Vec3,Vec4};
use rand::Rng;
use crate::trace;
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
    pub cam: Camera2D,
    pub rotation: f32,
    pub texture: Texture,
}

impl RenderContext {
    pub fn init() -> Self {
        unsafe { Renderer2D::init(); }

        let cam = Camera2D::new(-1.6, 1.5, -0.9, 0.9);
        let texture = unsafe { Texture::new(String::from("/home/anthony/.alkahest/projects/main/assets/mandark.png")) };

        RenderContext { cam, rotation: 0., texture }
    }

    pub fn draw(&mut self) {
        unsafe {
            command::set_clear_color(0.3, 0.3, 0.3, 1.);
            command::clear(gl::COLOR_BUFFER_BIT);
            // command::enable(gl::BLEND);
            // command::set_blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            self.cam.recalculate_matrices();
            self.rotation = self.rotation + 0.005;

            Renderer2D::begin_scene(&self.cam);

            // let mut rng = rand::thread_rng();
            // for x in 0..100 {
                // for y in 0..100 {
                    // let pos = Vec3::new((x as f32 / 100.) - 0.5, (y as f32 / 100.) - 0.5, 0.);
                    // let color = Vec4::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 1.);
                    // Renderer2D::draw_quad(pos, Vec2::new(0.01, 0.01), 0., color, None);
                // }
            // }

            // for x in 0..10 {
                // for y in 0..10 {
                    // let pos = Vec3::new((x as f32 / 10.) - 0.5, (y as f32 / 10.) - 0.5, 0.);
                    // Renderer2D::draw_quad(pos, Vec2::new(0.1, 0.1), 0., Vec4::new(1.,1.,1.,1.), Some(&self.texture));
                // }
            // }

            // Renderer2D::draw_quad(Vec3::new(0.2, 0.2, 0.), Vec2::new(0.4, 0.4), self.rotation, Vec4::new(0.8, 0.2, 0.3, 1.), None);
            // Renderer2D::draw_quad(Vec3::new(-0.3, -0.1, 0.), Vec2::new(0.6, 0.3), 0., Vec4::new(0.1, 0.2, 0.7, 1.), None);
            Renderer2D::draw_quad(Vec3::new(0.6, 0.4, 0.), Vec2::new(0.5, 0.5), 0., Vec4::new(1., 1., 1., 1.), Some(&self.texture));

            Renderer2D::end_scene();
        }        
    }

    pub fn cleanup(&self) {
        unsafe { Renderer2D::cleanup(); }
    }
}
