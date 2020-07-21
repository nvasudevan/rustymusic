use std::fmt;
use std::ops::Sub;

use rodio::source::SineWave;
use std::collections::HashMap;
use crate::SWARS;
use std::fmt::{Formatter, Error};

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert("-DHA", Hertz(233.08));
    swars.insert("-Ni", Hertz(246.94));
    swars.insert("-NI", Hertz(261.63));

    swars.insert("SA", Hertz(277.18));
    swars.insert("Re", Hertz(293.66));
    swars.insert("RE", Hertz(311.13));
    swars.insert("Ga", Hertz(329.63));
    swars.insert("GA", Hertz(349.23));
    swars.insert("MA", Hertz(369.99));
    swars.insert("MA'", Hertz(392.00));
    swars.insert("PA", Hertz(415.30));
    swars.insert("Dha", Hertz(440.0));
    swars.insert("DHA", Hertz(466.16));
    swars.insert("Ni", Hertz(493.88));
    swars.insert("NI", Hertz(523.25));
    swars.insert("SA+", Hertz(554.37));

    swars
}

pub const BASE_SWAR_INTERVAL: u64 = 1;

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
        Self("SA".to_string())
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(p.hertz().unwrap().0 as u32)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}
