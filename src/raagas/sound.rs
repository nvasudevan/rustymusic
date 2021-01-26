use rodio::{source::SineWave, OutputStreamHandle, Sink};
use std::fmt::Formatter;
use std::fmt;

use crate::raagas::constants;



pub struct TimedSink {
    pub sink: Sink,
    pub duration: f32
}

impl TimedSink {
    pub fn new(sink: Sink, duration: f32) -> Self {
        TimedSink {
            sink,
            duration
        }
    }
}

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

// impl FromStr for SineWave {
//     type Err = std::error::Error;
//
//     fn from_str(swar: &str) -> Result<Self, Self::Err>  {
//         return match constants::SWARS.get(swar) {
//             Ok(hz) => {
//                 Ok(SineWave::new(hz.freq))
//             },
//             _ => {
//                 // SineWave::new(Pitch::default().hertz().unwrap().freq as u32)
//                 Err(format!("Can't parse {} as a swar", swar))
//             }
//         }
//
//         }
// }

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hz = constants::SWARS.get(&*self.0);
        let _hz = hz.unwrap();
        let tone = _hz.tone();
        write!(f, "{}[{}]", self.0, tone)
    }
}
