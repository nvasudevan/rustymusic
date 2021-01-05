use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::io::BufReader;
use std::hash::{Hash, Hasher};

use rodio::{source::SineWave, Device, Sink, Source};

use crate::raagas::utils;
use crate::{SWARS};
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;

pub const BPS: f32 = 0.5; // equivalent to 120 BPM
pub const CONF_DIR: &str = "./config";
pub const BEATMP3: (&str, f32) = ("./samples/1beat.mp3", BPS);
pub const TIHAYI_TIMES: i8 = 3;
pub const KAN_SWAR_BEAT_COUNT: f32 = 0.2;

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert(".P", Hertz::new(207.65, "G#".to_string()));
    swars.insert(".d", Hertz::new(220.00, "A".to_string()));
    swars.insert(".D", Hertz::new(233.08, "A#".to_string()));
    swars.insert(".n", Hertz::new(246.94, "B".to_string()));
    swars.insert(".N", Hertz::new(261.63, "C".to_string()));

    swars.insert("S", Hertz::new(277.18, "C#".to_string()));
    swars.insert("r", Hertz::new(293.66, "D".to_string()));
    swars.insert("R", Hertz::new(311.13, "D#".to_string()));
    swars.insert("g", Hertz::new(329.63, "E".to_string()));
    swars.insert("G", Hertz::new(349.23, "F".to_string()));
    swars.insert("M", Hertz::new(369.99, "F#".to_string()));
    swars.insert("M'", Hertz::new(392.00, "G".to_string()));
    swars.insert("P", Hertz::new(415.30, "G#".to_string()));
    swars.insert("d", Hertz::new(440.0, "A".to_string()));
    swars.insert("D", Hertz::new(466.16, "A#".to_string()));
    swars.insert("n", Hertz::new(493.88, "B".to_string()));
    swars.insert("N", Hertz::new(523.25, "C".to_string()));
    swars.insert("S.", Hertz::new(554.37, "C#".to_string()));
    swars.insert("r.", Hertz::new(587.33, "D".to_string()));
    swars.insert("R.", Hertz::new(622.25, "D#".to_string()));
    swars.insert("g.", Hertz::new(659.25, "E".to_string()));
    swars.insert("G.", Hertz::new(698.46, "F".to_string()));
    swars.insert("M.", Hertz::new(739.99, "F#".to_string()));
    swars.insert("M'.", Hertz::new(783.99, "G".to_string()));

    swars
}

pub struct AudioDevice {
    pub(crate) dev: Device,
    vol: f32,
}

impl AudioDevice {
    pub fn new(dev: Device, vol: f32) -> AudioDevice {
        AudioDevice { dev, vol }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hertz {
    freq: f64,
    tone: String
}

impl Hertz {
    pub fn new(freq: f64, tone: String) -> Self {
        Hertz {
            freq,
            tone
        }
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
        let hz = SWARS.get(&*self.0);
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
        let hz = SWARS.get(&*self.0);
        let _hz = hz.unwrap();
        let tone = _hz.tone();
        write!(f, "{}[{}]", self.0, tone)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Swar {
    pub pitch: Option<Pitch>,
    pub beat_cnt: f32,
}

impl Swar {
    pub fn new(pitch: Pitch, beat_cnt: f32) -> Swar {
        Swar {
            pitch: Some(pitch),
            beat_cnt,
        }
    }

    pub fn pitch(&self) -> Option<Pitch> {
        self.pitch.clone()
    }

    pub fn beat_count(&self) -> f32 {
        self.beat_cnt
    }
}

impl fmt::Display for Swar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let _cnt = self.beat_cnt as usize;
        let dash = match _cnt {
            1 => String::from(" "),
            2..=8 => (0..(_cnt - 1))
                .map(|_| " - ")
                .collect::<String>(),
            _ => String::new(),
        };

        let _s = match &self.pitch {
            Some(sw) => format!("{}{}", sw, dash),
            _ => String::new(),
        };
        write!(f, "{}", _s)
    }
}

pub trait Melody {
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        mix: bool,
        n: i8,
    );
}

impl Melody for Swar {
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        mix: bool,
        _n: i8,
    ) {
        let sink = Sink::new(&dev.dev);
        match beat_src {
            Some(src) => {
                match &self.pitch {
                    // play swar with taal
                    Some(p) => {
                        let sinew = SineWave::from(p.to_owned());
                        sink.append(src.mix(sinew));
                    },
                    _ => {
                        // play taal
                        sink.append(src);
                    }
                }
            },
            _ => {
                // play swar
                match &self.pitch {
                    Some(p) => {
                        let sinew = SineWave::from(p.to_owned());
                        sink.append(sinew);
                    },
                    _ => {}
                }
            }
        }

        sink.set_volume(*&dev.vol as f32);
        sink.play();
        utils::delay(self.beat_cnt * BPS);
        sink.stop();
    }
}

pub struct Taal {
   taal: Repeat<TakeDuration<Decoder<BufReader<File>>>>,
   bps: f32
}

impl Taal {
    pub fn new(taal: Repeat<TakeDuration<Decoder<BufReader<File>>>>, bps: f32) -> Self {
        Taal { taal, bps }
    }

    pub fn play(&self, sink: Sink) {
        sink.append(self.taal.clone());
    }
}