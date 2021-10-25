use std::collections::VecDeque;
use std::f32::consts::PI;

use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{self, GamepadAxis, GamepadButton, Key};
use tetra::math::Vec2;
use tetra::Context;

use crate::assets::Assets;
use crate::SCREEN_HEIGHT;

pub struct Star {
    pub position: Vec2<f32>,
    velocity: Vec2<f32>,
    rotation: f32,
    trail: VecDeque<f32>,
    trail_timer: i32,
}

impl Star {
    const MOVEMENT_ACC: f32 = 0.1;
    const MOVEMENT_DEC: f32 = 0.2;
    const MOVEMENT_FRC: f32 = 0.05;
    const ROTATION_SPEED: f32 = 0.05;

    const TRAIL_RATE: i32 = 1;
    const TRAIL_LENGTH: usize = 32;
    const TRAIL_DISTANCE: f32 = 100.0;
    const TRAIL_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

    const DEADZONE: f32 = 0.05;

    pub fn new(position: Vec2<f32>) -> Star {
        let mut trail = VecDeque::with_capacity(Self::TRAIL_LENGTH);

        for _ in 0..Self::TRAIL_LENGTH {
            trail.push_front(position.y);
        }

        Star {
            position,
            velocity: Vec2::zero(),
            rotation: 0.0,
            trail,
            trail_timer: 0,
        }
    }

    pub fn bounds(&self, assets: &Assets) -> Rectangle {
        let width = assets.star.width() as f32;
        let height = assets.star.height() as f32;
        let x = self.position.x - width / 2.0;
        let y = self.position.y - height / 2.0;

        Rectangle::new(x, y, width, height)
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let mut axis = input::get_gamepad_axis_position(ctx, 0, GamepadAxis::LeftStickY);

        if input::is_key_down(ctx, Key::W)
            || input::is_key_down(ctx, Key::Up)
            || input::is_gamepad_button_down(ctx, 0, GamepadButton::Up)
        {
            axis = -1.0;
        } else if input::is_key_down(ctx, Key::S)
            || input::is_key_down(ctx, Key::Down)
            || input::is_gamepad_button_down(ctx, 0, GamepadButton::Down)
        {
            axis = 1.0;
        }

        if axis.abs() > Self::DEADZONE {
            if self.velocity.y == 0.0 || self.velocity.y.signum() == axis.signum() {
                self.velocity.y += Self::MOVEMENT_ACC * axis;
            } else {
                self.velocity.y += Self::MOVEMENT_DEC * axis;
            }
        } else {
            self.velocity -= self.velocity * Self::MOVEMENT_FRC;
        }

        self.position += self.velocity;
        self.rotation += Self::ROTATION_SPEED;

        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
        } else if self.position.y >= SCREEN_HEIGHT {
            self.position.y = SCREEN_HEIGHT;
            self.velocity.y = 0.0;
        }

        self.trail_timer += 1;

        if self.trail_timer == Self::TRAIL_RATE {
            self.trail.pop_back();
            self.trail.push_front(self.position.y);

            self.trail_timer = 0;
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, blend: f32) {
        let position = self.position + self.velocity * blend;
        let rotation = self.rotation + Self::ROTATION_SPEED * blend;
        let pulse = f32::sin(PI * rotation) / 4.0;

        for (i, &y) in self.trail.iter().enumerate() {
            let fade = i as f32 / Self::TRAIL_LENGTH as f32;
            let inv_fade = 1.0 - fade;

            let x = position.x - (fade * Self::TRAIL_DISTANCE);

            let params = DrawParams::new()
                .position(Vec2::new(x, y))
                .origin(Vec2::new(4.0, 4.0))
                .rotation(rotation - fade)
                .color(Self::TRAIL_COLOR.with_alpha(inv_fade))
                .scale(Vec2::new(inv_fade, inv_fade));

            assets.star.draw(ctx, params);
        }

        assets.star.draw(
            ctx,
            DrawParams::new()
                .position(position)
                .origin(Vec2::new(4.0, 4.0))
                .rotation(rotation)
                .scale(Vec2::new(1.0 + pulse, 1.0 + pulse)),
        );
    }
}
