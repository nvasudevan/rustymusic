
use std::fmt;


use std::io::{Write, BufReader};
use std::fmt::Formatter;
use std::collections::HashMap;
use std::ops::Sub;

use rodio::{Sink, source::SineWave, Device};
use crate::raagas::utils;
use crate::SWARS;

pub const BPM: f32 = 1.0;
pub const VOLUME_LEVEL: f32 = 2.0;
pub const CONF_DIR: &str = "./config";

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert("_DHA", Hertz(233.08));
    swars.insert("_.NI", Hertz(246.94)); //komal ni in lower octave
    swars.insert("_NI", Hertz(261.63));

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
pub struct Swar {
    pub swar: Option<Pitch>,
    pub beat_cnt: u64,
}

impl fmt::Display for Swar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dash = (0..(self.beat_cnt-1)).map(|_| " - ").collect::<String>();
        let mut _s = "".to_string();
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
pub struct SwarBlock(pub Vec<Swar>);

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
    aroha: Vec<Swar>,
    avroha: Vec<Swar>,
    pakad: Vec<SwarBlock>,
    alankars: Vec<Vec<Swar>>,
}

impl Raag {
    pub fn new(name: String,
               aroha: Vec<Swar>,
               avroha: Vec<Swar>,
               pakad: Vec<SwarBlock>,
               alankars: Vec<Vec<Swar>>) -> Raag {
        Raag {
            name,
            aroha,
            avroha,
            pakad,
            alankars,
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn aroha(&self) -> &Vec<Swar> {
        &self.aroha
    }

    pub fn avroha(&self) -> &Vec<Swar> {
        &self.avroha
    }

    pub fn pakad(&self) -> &Vec<SwarBlock> {
        &self.pakad
    }

    pub fn alankars(&self) -> &Vec<Vec<Swar>> {
        &self.alankars
    }

    fn play_aroha(&self, dev: &Device) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        for sw in self.aroha() {
            print!("{} ", sw);
            io_flush();
            play_swar(&dev, &sw);
        }
    }

    fn play_avroha(&self, dev: &Device) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        for sw in self.avroha() {
            print!("{} ", sw);
            io_flush();
            play_swar(&dev, &sw);
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
            for sw in &blk.0 {
                print!("{} ", sw);
                io_flush();
                play_swar(&dev, &sw);
            }
        }
    }

    fn play_alankars(&self, dev: &Device) {
        println!("\n=> Playing alankars for raag: {}", self.name());
        for alankar in self.alankars() {
            for sw in alankar {
                print!("{} ", sw);
                io_flush();
                play_swar(&dev, &sw);
            }
            println!();
        }
    }
}

impl Melody for Raag {
    fn play(&self, dev: &Device) {
        let gap: i32 = 4; //no of beats
        // self.play_aroha(dev);
        // utils::delay((((gap as f32) * BPM) as u64));
        // self.play_avroha(&dev);
        // utils::delay((((gap as f32) * BPM) as u64));
        self.play_pakad(&dev);
        utils::delay(((gap as f32) * BPM) as u64);
        self.play_alankars(&dev);
    }
}


pub fn play_swar(dev: &Device, sw: &Swar) {
    let sink = Sink::new(&dev);
    match &sw.swar {
        Some(p) =>  {
            let sinew = SineWave::from(p.to_owned());
            sink.append(sinew);
            sink.play();
            sink.set_volume(VOLUME_LEVEL);
            utils::delay(sw.beat_cnt * BPM as u64);
            sink.stop();
        },
        _ => {
            let f: std::fs::File = std::fs::File::open("./samples/beep.wav").unwrap();
            let beep = rodio::play_once(&dev, BufReader::new(f)).unwrap();
            beep.set_volume(VOLUME_LEVEL);
            let bt_cnt = 2;
            utils::delay((bt_cnt as f32 * BPM) as u64);
            beep.stop();
        }
    }
}
