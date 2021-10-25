use tetra::math::Vec2;
use tetra::Context;

use crate::assets::Assets;

pub struct Backdrop {
    position: Vec2<f32>,
}

impl Backdrop {
    pub fn new() -> Backdrop {
        Backdrop {
            position: Vec2::zero(),
        }
    }

    pub fn update(&mut self, speed: f32) {
        self.position.x = (self.position.x - speed / 2.0) % 1280.0;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, speed: f32, blend: f32) {
        let position = (self.position - Vec2::new(speed / 2.0 * blend, 0.0)) % 1280.0;

        assets.backdrop.draw(ctx, position);
    }
}
