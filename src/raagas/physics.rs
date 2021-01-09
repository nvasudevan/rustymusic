use rodio::{source::SineWave, Device, Sink, Source};
use rodio::default_output_device;
use std::fmt::Formatter;
use std::fmt;

use crate::raagas::constants;

pub struct AudioDevice {
    pub(crate) dev: Device,
    pub(crate) vol: f32,
}

impl AudioDevice {
    pub fn new() -> AudioDevice {
        let dev = default_output_device().unwrap();
        AudioDevice { dev, vol: constants::VOL }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hertz {
    freq: f64,
    tone: String,
}

impl Hertz {
    pub fn new(freq: f64, tone: String) -> Self {
        Hertz { freq, tone }
    }

    pub fn freq(&self) -> f64 {
        self.freq
    }

    pub fn tone(&self) -> &str {
        self.tone.as_ref()
    }
}

// impl Sub for Hertz {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Hertz::new(self.freq - rhs.freq, self)
//     }
// }

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.freq
    }
}

// impl From<f64> for Hertz {
//     fn from(f: f64) -> Self {
//         Hertz {
//             freq: f,
//             tone: "".to_string(),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Pitch(String);

impl Pitch {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn hertz(&self) -> Option<Hertz> {
        let hz = constants::SWARS.get(&*self.0);
        let _hz = hz.unwrap();
        Some(_hz.to_owned())
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self("S".to_string())
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(p.hertz().unwrap().freq as u32)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hz = constants::SWARS.get(&*self.0);
        let _hz = hz.unwrap();
        let tone = _hz.tone();
        write!(f, "{}[{}]", self.0, tone)
    }
}
