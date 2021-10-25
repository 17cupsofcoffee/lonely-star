mod assets;
mod audio;
mod objects;

use tetra::audio::Sound;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{self, Color};
use tetra::math::Vec2;
use tetra::time;
use tetra::{Context, Event, State};

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use crate::assets::Assets;
use crate::audio::{ChordPicker, NotePicker};
use crate::objects::{Backdrop, Gate, Orb, Spawner, Star};

pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 360.0;

pub struct GameState {
    scaler: ScreenScaler,
    assets: Assets,
    rng: Pcg64Mcg,

    backdrop: Backdrop,
    star: Star,
    gates: Vec<Gate>,
    orbs: Vec<Orb>,

    note_picker: NotePicker,
    chord_picker: ChordPicker,

    speed: f32,

    spawner: Spawner,

    score: i32,
    score_text: Text,

    lives: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Sound::new("./resources/drone.wav")?.repeat_with(ctx, 0.5, 1.0)?;

        Ok(GameState {
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            assets: Assets::new(ctx)?,
            rng: Pcg64Mcg::from_entropy(),

            backdrop: Backdrop::new(),
            star: Star::new(Vec2::new(SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 2.0)),
            gates: Vec::new(),
            orbs: Vec::new(),

            note_picker: NotePicker::new()?,
            chord_picker: ChordPicker::new()?,

            speed: 2.0,

            spawner: Spawner::new(),

            score: 0,
            score_text: Text::new("00000000", Font::vector(ctx, "./resources/m5x7.ttf", 16.0)?),

            lives: 5,
        })
    }

    fn update_score(&mut self, points: i32) {
        self.score += points;
        self.score_text.set_content(format!("{:08}", self.score));
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.lives > 0 {
            let (gate, orb) = self.spawner.check(&mut self.rng, self.speed);

            if let Some(g) = gate {
                self.gates.push(g);
            }

            if let Some(o) = orb {
                self.orbs.push(o);
            }

            self.backdrop.update(self.speed);
            self.star.update(ctx);

            let mut points = 0;

            for gate in &mut self.gates {
                gate.update(self.speed);

                if !gate.hit && !gate.missed {
                    if gate.collide(&self.assets, &self.star) {
                        self.chord_picker.play(ctx, true)?;
                        self.speed += 0.05;

                        gate.hit = true;
                        points += 50;
                    } else if gate.position.x < self.star.position.x {
                        self.chord_picker.play(ctx, false)?;

                        gate.missed = true;
                        self.lives -= 1;
                    }
                }
            }

            for orb in &mut self.orbs {
                orb.update(self.speed);

                if !orb.hit && orb.collide(&self.star) {
                    self.note_picker.play(ctx, &mut self.rng)?;

                    orb.hit = true;
                    points += 10;
                }
            }

            if points > 0 {
                self.update_score(points);
            }

            if self.lives <= 0 {
                self.speed = 0.0;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::WHITE);

        let blend = time::get_blend_factor(ctx);

        self.backdrop.draw(ctx, &self.assets, self.speed, blend);

        for gate in &self.gates {
            gate.draw_back(ctx, &self.assets, self.speed, blend);
        }

        if self.lives > 0 {
            self.star.draw(ctx, &self.assets, blend);
        }

        for orb in &self.orbs {
            orb.draw(ctx, &self.assets, self.speed, blend);
        }

        for gate in &self.gates {
            gate.draw_front(ctx, &self.assets, self.speed, blend);
        }

        for i in 0..5 {
            let texture = if i < self.lives {
                &self.assets.heart
            } else {
                &self.assets.heart_broken
            };

            texture.draw(ctx, Vec2::new(8.0 + (12.0 * i as f32), 8.0));
        }

        self.score_text.draw(ctx, Vec2::new(8.0, 16.0));

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
