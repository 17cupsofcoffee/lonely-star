use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::math::Vec2;
use tetra::Context;

use crate::assets::Assets;
use crate::objects::Star;

pub struct Gate {
    pub position: Vec2<f32>,
    pub hit: bool,
    pub missed: bool,
}

impl Gate {
    pub fn new(position: Vec2<f32>) -> Gate {
        Gate {
            position,
            hit: false,
            missed: false,
        }
    }

    pub fn bounds(&self, assets: &Assets) -> Rectangle {
        let width = assets.gate_back.width() as f32;
        let height = assets.gate_back.height() as f32;
        let x = self.position.x - width / 2.0;
        let y = self.position.y - height / 2.0;

        Rectangle::new(x, y, width, height)
    }

    pub fn update(&mut self, speed: f32) {
        self.position.x -= speed;
    }

    pub fn draw_back(&self, ctx: &mut Context, assets: &Assets, speed: f32, blend: f32) {
        self.draw(ctx, assets, true, speed, blend);
    }

    pub fn draw_front(&self, ctx: &mut Context, assets: &Assets, speed: f32, blend: f32) {
        self.draw(ctx, assets, false, speed, blend);
    }

    pub fn collide(&self, assets: &Assets, star: &Star) -> bool {
        !self.hit && self.bounds(assets).intersects(&star.bounds(assets))
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, back: bool, speed: f32, blend: f32) {
        let position = self.position - Vec2::new(speed * blend, 0.0);

        let texture = if back {
            &assets.gate_back
        } else {
            &assets.gate_front
        };

        texture.draw(
            ctx,
            DrawParams::new()
                .position(position)
                .origin(Vec2::new(
                    texture.width() as f32 / 2.0,
                    texture.height() as f32 / 2.0,
                ))
                .color(if self.missed {
                    Color::rgb(0.2, 0.2, 0.2)
                } else {
                    Color::WHITE
                }),
        );
    }
}

pub struct Orb {
    pub position: Vec2<f32>,
    pub fade: f32,
    pub hit: bool,
}

impl Orb {
    pub fn new(position: Vec2<f32>) -> Orb {
        Orb {
            position,
            fade: 0.0,
            hit: false,
        }
    }

    pub fn update(&mut self, speed: f32) {
        if self.hit && self.fade < 1.0 {
            self.fade = f32::min(1.0, self.fade + 0.05);
        }

        self.position.x -= speed;
    }

    pub fn collide(&self, star: &Star) -> bool {
        !self.hit && self.position.distance(star.position) < 20.0
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, speed: f32, blend: f32) {
        let position = self.position - Vec2::new(speed * blend, 0.0);
        let inv_fade = 1.0 - self.fade;

        assets.orb.draw(
            ctx,
            DrawParams::new()
                .position(position)
                .scale(Vec2::new(1.0 + (2.0 * self.fade), 1.0 + (2.0 * self.fade)))
                .origin(Vec2::new(
                    assets.orb.width() as f32 / 2.0,
                    assets.orb.height() as f32 / 2.0,
                ))
                .color(Color::rgba(inv_fade, inv_fade, inv_fade, inv_fade)),
        );
    }
}
