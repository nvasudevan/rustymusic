use crate::SWARS;

use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use std::io::{Write, BufReader};
use std::fmt::Formatter;
use std::collections::HashMap;
use std::ops::Sub;

use rodio::{Sink, source::SineWave, Device};

pub const BASE_SWAR_INTERVAL: u64 = 1;

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert("-DHA", Hertz(233.08));
    swars.insert("-Ni", Hertz(246.94));
    swars.insert("-NI", Hertz(261.63));

    swars.insert("SA", Hertz(277.18));
    swars.insert(".RE", Hertz(293.66));
    swars.insert("RE", Hertz(311.13));
    swars.insert(".GA", Hertz(329.63));
    swars.insert("GA", Hertz(349.23));
    swars.insert("MA", Hertz(369.99));
    swars.insert("MA'", Hertz(392.00));
    swars.insert("PA", Hertz(415.30));
    swars.insert(".DHA", Hertz(440.0));
    swars.insert("DHA", Hertz(466.16));
    swars.insert(".NI", Hertz(493.88));
    swars.insert("NI", Hertz(523.25));
    swars.insert("SA+", Hertz(554.37));

    swars
}

fn io_flush() {
    match std::io::stdout().flush() {
        Ok(()) => {},
        _ => { panic!("I/O couldn't be flushed to terminal!")}
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
        Self("SA".to_string())
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

#[derive(Debug, Clone)]
pub struct Beat {
    pub swar: Option<Pitch>,
    pub long: u64,
}

impl fmt::Display for Beat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dash = (0..(self.long-1)).map(|_| " - ").collect::<String>();
        let mut _s = ".".to_string();
        match &self.swar {
            Some(sw) => {
                _s  = format!("{}{}", sw, dash);
            },
            _ => {}
        }
        write!(f, "{}", _s)
    }
}

pub trait Melody {
    fn play(&self, dev: &Device);
}

#[derive(Debug, Clone)]
pub struct SwarBlock(pub Vec<Beat>);

impl Melody for SwarBlock {
    fn play(&self, dev: &Device) {
       for bt in &self.0 {
           print!("{} ", bt);
           io_flush();
           play_swar(&dev, &bt);
       }
    }
}

#[derive(Debug, Clone)]
pub struct Raag {
    name: String,
    aroha: Vec<Beat>,
    avroha: Vec<Beat>,
    pakad: Vec<SwarBlock>
}

impl Raag {
    pub fn new(name: String, aroha: Vec<Beat>, avroha: Vec<Beat>, pakad: Vec<SwarBlock>) -> Raag {
        Raag {
            name,
            aroha,
            avroha,
            pakad,
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn aroha(&self) -> &Vec<Beat> {
        &self.aroha
    }

    pub fn avroha(&self) -> &Vec<Beat> {
        &self.avroha
    }

    pub fn pakad(&self) -> &Vec<SwarBlock> {
        &self.pakad
    }

    fn play_aroha(&self, dev: &Device) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        for bt in self.aroha() {
            print!("{} ", bt);
            io_flush();
            play_swar(&dev, &bt);
        }
    }

    fn play_avroha(&self, dev: &Device) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        for bt in self.avroha() {
            print!("{} ", bt);
            io_flush();
            play_swar(&dev, &bt);
        }
    }

    fn play_pakad(&self, dev: &Device) {
        println!("\n=> Playing pakad for raag: {}", self.name());
        let mut _comma: bool = false;
        for blk in self.pakad() {
            if _comma {
                print!(", ");
                io_flush();
            }
            _comma = true;
            for bt in &blk.0 {
                print!("{} ", bt);
                io_flush();
                play_swar(&dev, &bt);
            }
        }
    }
}


impl Melody for Raag {
    fn play(&self, dev: &Device) {
        self.play_aroha(dev);
        delay(2);
        self.play_avroha(&dev);
        delay(2);
        self.play_pakad(&dev);
    }
}

pub fn delay(t: u64) {
    sleep(Duration::from_secs(t));
}

pub fn play_swar(dev: &Device, bt: &Beat) {
    let sink = Sink::new(&dev);
    match &bt.swar {
        Some(p) =>  {
            let sw = SineWave::from(p.to_owned());
            sink.append(sw);
            sink.play();
            delay(bt.long);
            sink.stop();
        },
        _ => {
            let f: std::fs::File = std::fs::File::open("./samples/beep.wav").unwrap();
            let beep = rodio::play_once(&dev, BufReader::new(f)).unwrap();
            beep.set_volume(0.2);
            delay(BASE_SWAR_INTERVAL);
            beep.stop();
        }
    }
}
