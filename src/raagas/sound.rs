use rodio::{source::SineWave, OutputStreamHandle, Sink};
use std::fmt::Formatter;
use std::fmt;

use crate::raagas::constants;

pub struct AudioDevice {
    pub(crate) out_stream_handle: OutputStreamHandle,
    pub(crate) vol: f32,
}

impl AudioDevice {
    pub fn new(out_stream_handle: OutputStreamHandle) -> AudioDevice {
        AudioDevice { out_stream_handle, vol: constants::VOL }
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

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.freq
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pitch(String);

impl Pitch {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn hertz(&self) -> Option<Hertz> {
        if let Some(hz) = constants::SWARS.get(&*self.0) {
            return Some(hz.to_owned());
        }

        None
    }

    pub fn from_swar(s: &str) -> SineWave {
        return match constants::SWARS.get(s) {
            Some(hz) => {
                SineWave::new(hz.freq as u32)
            },
            _ => {
                SineWave::new(Pitch::default().hertz().unwrap().freq as u32)
            }
        }
    }

    pub fn to_sinewave(&self) -> Option<SineWave> {
        if let Some(hz) = self.hertz() {
            return Some(SineWave::new(hz.freq as u32));
        }
        None
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self("S".to_string())
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        return SineWave::new(p.hertz().unwrap().freq as u32);
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut tone = "";
        if let Some(hz) = constants::SWARS.get(&*self.0) {
            tone = hz.tone();
        }
        // write!(f, "{}[{}]", self.0, tone)
        write!(f, "{}", self.0)
    }
}
