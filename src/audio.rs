use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand_pcg::Pcg64Mcg;
use tetra::audio::Sound;
use tetra::Context;

pub struct NotePicker {
    sounds: Vec<Sound>,
    last_note: usize,
}

impl NotePicker {
    pub fn new() -> tetra::Result<NotePicker> {
        Ok(NotePicker {
            sounds: vec![
                Sound::new("./resources/a#3.wav")?,
                Sound::new("./resources/c4.wav")?,
                Sound::new("./resources/c#4.wav")?,
                Sound::new("./resources/d#4.wav")?,
                Sound::new("./resources/f4.wav")?,
                Sound::new("./resources/f#4.wav")?,
                Sound::new("./resources/g#4.wav")?,
                Sound::new("./resources/a#4.wav")?,
            ],

            last_note: 0,
        })
    }

    pub fn play(&mut self, ctx: &mut Context, rng: &mut Pcg64Mcg) -> tetra::Result {
        let index = WeightedIndex::new(&[
            self.weight(0, 3), // A#3
            self.weight(1, 1), // C4
            self.weight(2, 1), // C#4
            self.weight(3, 1), // D#4
            self.weight(4, 2), // F4
            self.weight(5, 1), // F#4
            self.weight(6, 1), // G#4
            self.weight(7, 1), // A#4
        ])
        .unwrap();

        let note = index.sample(rng);

        self.sounds[note].play_with(ctx, 0.5, 1.0)?;

        self.last_note = note;

        Ok(())
    }

    fn weight(&self, note: usize, base: i32) -> i32 {
        if self.last_note == note {
            return 0;
        }

        base
    }
}

pub struct ChordPicker {
    ring1: Sound,
    ring2: Sound,
    ring3: Sound,
    next_sound: i32,
}

impl ChordPicker {
    pub fn new() -> tetra::Result<ChordPicker> {
        Ok(ChordPicker {
            ring1: Sound::new("./resources/ring1.wav")?,
            ring2: Sound::new("./resources/ring2.wav")?,
            ring3: Sound::new("./resources/ring3.wav")?,
            next_sound: 0,
        })
    }

    pub fn play(&mut self, ctx: &mut Context, hit: bool) -> tetra::Result {
        let pitch = if hit { 1.0 } else { 0.25 };

        match self.next_sound {
            0 => &self.ring1,
            1 => &self.ring2,
            2 => &self.ring3,
            _ => unreachable!(),
        }
        .play_with(ctx, 0.5, pitch)?;

        self.next_sound = (self.next_sound + 1) % 3;

        Ok(())
    }
}
