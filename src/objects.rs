mod backdrop;
mod gate;
mod star;

pub use backdrop::*;
pub use gate::*;
pub use star::*;

use rand::Rng;
use rand_pcg::Pcg64Mcg;
use tetra::math::Vec2;

use crate::SCREEN_HEIGHT;

pub struct Spawner {
    gate_timer: f32,
    orb_timer: f32,
    last_y: Option<f32>,
}

impl Spawner {
    const GATE_RATE: f32 = 400.0;
    const ORB_RATE: f32 = Self::GATE_RATE / 4.0;
    const ORB_CHANCE: f64 = 0.6;

    const SPAWN_X: f32 = 700.0;
    const MIN_Y: f32 = 45.0;
    const MAX_Y: f32 = 315.0;
    const MAX_Y_CHANGE: f32 = 180.0;

    pub fn new() -> Spawner {
        Spawner {
            gate_timer: Self::GATE_RATE,
            orb_timer: 0.0,
            last_y: None,
        }
    }

    pub fn check(&mut self, rng: &mut Pcg64Mcg, clock: f32) -> (Option<Gate>, Option<Orb>) {
        self.gate_timer += clock;
        self.orb_timer += clock;

        let gate_ready = self.gate_timer >= Self::GATE_RATE;
        let orb_ready = self.orb_timer >= Self::ORB_RATE;

        if !gate_ready && !orb_ready {
            return (None, None);
        }

        let y = match self.last_y {
            Some(last_y) => {
                let min_y = f32::max(Self::MIN_Y, last_y - Self::MAX_Y_CHANGE);
                let max_y = f32::min(Self::MAX_Y, last_y + Self::MAX_Y_CHANGE);

                rng.gen_range(min_y..=max_y)
            }
            None => SCREEN_HEIGHT / 2.0, // Always start in the center
        };

        self.last_y = Some(y);

        let spawn_pos = Vec2::new(Self::SPAWN_X, y);

        let gate = if gate_ready {
            self.gate_timer -= Self::GATE_RATE;
            Some(Gate::new(spawn_pos))
        } else {
            None
        };

        let note = if orb_ready {
            self.orb_timer -= Self::ORB_RATE;

            if rng.gen_bool(Self::ORB_CHANCE) {
                Some(Orb::new(spawn_pos))
            } else {
                None
            }
        } else {
            None
        };

        (gate, note)
    }
}
