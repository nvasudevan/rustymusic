use std::fmt;
use std::io::BufReader;
use std::fmt::Formatter;
use std::collections::HashMap;
use std::ops::Sub;

use rodio::{Sink, source::SineWave, Device};
use crate::raagas::utils;
use crate::SWARS;

pub const BPS: f32 = 0.5;  // equivalent to 120 BPM
pub const CONF_DIR: &str = "./config";

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
    dev: Device,
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
            beat_cnt
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
        let mut dash = String::new();
        if self.beat_cnt > 1.0 {
            dash = (0..(self.beat_cnt as usize-1)).map(|_| " - ").collect::<String>();
        }
        let mut _s = "".to_string();
        match &self.pitch {
            Some(sw) => {
                _s  = format!("{}{}", sw, dash);
            },
            _ => {}
        }
        write!(f, "{}", _s)
    }
}

pub trait Melody {
    fn play(&self, dev: &AudioDevice);
}

#[derive(Debug, Clone)]
pub struct SwarBlock(pub Vec<Swar>);

impl Melody for SwarBlock {
    fn play(&self, dev: &AudioDevice) {
       for bt in &self.0 {
           print!("{} ", bt);
           utils::io_flush();
           play_swar(&dev, &bt);
       }
    }
}

#[derive(Debug, Clone)]
pub struct Swarmaalika {
    pub sthayi: Vec<SwarBlock>,
    pub antara: Vec<SwarBlock>,
}

impl Swarmaalika {
    pub fn new(sthayi: Vec<SwarBlock>, antara: Vec<SwarBlock>) -> Self {
       Swarmaalika {
           sthayi,
           antara
       }
    }
}

impl Melody for Swarmaalika {
    fn play(&self, dev: &AudioDevice) {
        // play: sthayi, line A of sthayi, antara, line A of sthayi, tihayi
        println!("\nPlaying swarmaalika");
        let sthayi = &self.sthayi;
        let antara = &self.antara;
        // let _gap:i32 = 1;

        // play each line of sthayi twice
        for blk in sthayi {
            for _ in 0..2 {
                for sw in &blk.0 {
                    print!("{} ", sw);
                    utils::io_flush();
                    play_swar(&dev, &sw);
                }
                println!();
            }
        }
        println!();
        let line_a = sthayi.get(0).unwrap();
        for sw in &line_a.0 {
            print!("{} ", sw);
            utils::io_flush();
            play_swar(&dev, &sw);
        }
        println!();

        for blk in antara {
            for _ in 0..2 {
                for sw in &blk.0 {
                    print!("{} ", sw);
                    utils::io_flush();
                    play_swar(&dev, sw);
                }
                println!();
            }
        }
        println!();

        for sw in &line_a.0 {
            print!("{} ", sw);
            utils::io_flush();
            play_swar(&dev, sw);
        }
        println!();
        // tihayi is played n (=3) times
        let _n = 3;
        for _i in 0..3 {
            // we only play the first j beats
            let j = (line_a.0.len()/2)-1;
            let tihyai = &line_a.0[..j];
            for sw in tihyai {
                print!("{} ", sw);
                utils::io_flush();
                play_swar(&dev, &sw);
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone)]
pub struct Raag {
    name: String,
    aroha: Vec<Swar>,
    avroha: Vec<Swar>,
    pakad: Vec<SwarBlock>,
    alankars: Vec<Vec<Swar>>,
    swarmaalika: Swarmaalika,
}

impl Raag {
    pub fn new(name: String,
               aroha: Vec<Swar>,
               avroha: Vec<Swar>,
               pakad: Vec<SwarBlock>,
               alankars: Vec<Vec<Swar>>,
               swarmaalika: Swarmaalika) -> Raag {
        Raag {
            name,
            aroha,
            avroha,
            pakad,
            alankars,
            swarmaalika,
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

    pub fn swarmaalika(&self) -> &Swarmaalika {
        &self.swarmaalika
    }

    fn play_aroha(&self, dev: &AudioDevice) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        for sw in self.aroha() {
            print!("{} ", sw);
            utils::io_flush();
            play_swar(&dev, &sw);
        }
    }

    fn play_avroha(&self, dev: &AudioDevice) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        for sw in self.avroha() {
            print!("{} ", sw);
            utils::io_flush();
            play_swar(&dev, &sw);
        }
    }

    fn play_pakad(&self, dev: &AudioDevice) {
        println!("\n=> Playing pakad for raag: {}", self.name());
        let mut _comma: bool = false;
        for blk in self.pakad() {
            if _comma {
                print!(", ");
                utils::io_flush();
            }
            _comma = true;
            for sw in &blk.0 {
                print!("{} ", sw);
                utils::io_flush();
                play_swar(&dev, &sw);
            }
        }
    }

    #[allow(dead_code)]
    fn play_alankars(&self, dev: &AudioDevice) {
        println!("\n=> Playing alankars for raag: {}", self.name());
        for alankar in self.alankars() {
            for sw in alankar {
                print!("{} ", sw);
                utils::io_flush();
                play_swar(&dev, &sw);
            }
            println!();
        }
    }

}

impl Melody for Raag {
    fn play(&self, dev: &AudioDevice) {
        let gap: f32 = 1.0; //no of beats
        self.play_aroha(&dev);
        utils::delay(gap * BPS);
        self.play_avroha(&dev);
        utils::delay(gap * BPS);
        self.play_pakad(&dev);
        utils::delay(gap * BPS);
        self.swarmaalika.play(dev);
        utils::delay(gap * BPS);
        self.play_alankars(&dev);
    }
}

pub fn play_swar(dev: &AudioDevice, sw: &Swar) {
    let sink = Sink::new(&dev.dev);
    match &sw.pitch {
        Some(p) =>  {
            let sinew = SineWave::from(p.to_owned());
            sink.append(sinew);
            sink.set_volume(*&dev.vol as f32);
            sink.play();
            utils::delay(sw.beat_cnt * BPS);
            sink.stop();
        },
        _ => {
            let f: std::fs::File = std::fs::File::open("./samples/beep.wav").unwrap();
            let beep = rodio::play_once(&dev.dev, BufReader::new(f)).unwrap();
            sink.set_volume(*&dev.vol as f32);
            let bt_cnt = 2.0;
            utils::delay(bt_cnt * BPS);
            beep.stop();
        }
    }
}
