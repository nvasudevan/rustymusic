use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::io::BufReader;
use std::ops::Sub;

use rodio::{source::SineWave, Device, Sink, Source};

use crate::raagas::utils;
use crate::SWARS;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;

pub const BPS: f32 = 0.5; // equivalent to 120 BPM
pub const CONF_DIR: &str = "./config";
pub const BEATMP3: (&str, f32) = ("./samples/1beat.mp3", BPS);
pub const TIHAYI_TIMES: i8 = 3;

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert(".P", Hertz(207.65));
    swars.insert(".d", Hertz(220.00));
    swars.insert(".D", Hertz(233.08));
    swars.insert(".n", Hertz(246.94)); //komal ni in lower octave
    swars.insert(".N", Hertz(261.63));

    swars.insert("S", Hertz(277.18)); // C#
    swars.insert("r", Hertz(293.66));
    swars.insert("R", Hertz(311.13));
    swars.insert("g", Hertz(329.63));
    swars.insert("G", Hertz(349.23));
    swars.insert("M", Hertz(369.99));
    swars.insert("M'", Hertz(392.00));
    swars.insert("P", Hertz(415.30));
    swars.insert("d", Hertz(440.0));
    swars.insert("D", Hertz(466.16));
    swars.insert("n", Hertz(493.88));
    swars.insert("N", Hertz(523.25));
    swars.insert("S.", Hertz(554.37));
    swars.insert("r.", Hertz(587.33));
    swars.insert("R.", Hertz(622.25));
    swars.insert("g.", Hertz(659.25));
    swars.insert("G.", Hertz(698.46));
    swars.insert("M.", Hertz(739.99));
    swars.insert("M'.", Hertz(783.99));

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hertz(pub f64);

impl Sub for Hertz {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Hertz(self.0 - rhs.0)
    }
}

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.0
    }
}

impl From<f64> for Hertz {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

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
        SineWave::new(p.hertz().unwrap().0 as u32)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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