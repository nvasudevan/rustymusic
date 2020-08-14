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
        let mut dash = String::new();
        if self.beat_cnt > 1.0 {
            dash = (0..(self.beat_cnt as usize - 1))
                .map(|_| " - ")
                .collect::<String>();
        }
        let mut _s = "".to_string();
        match &self.pitch {
            Some(sw) => {
                _s = format!("{}{}", sw, dash);
            }
            _ => {}
        }
        write!(f, "{}", _s)
    }
}

pub trait Melody {
    fn play(&self, dev: &AudioDevice, beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>, n: i8);
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

impl SwarBlock {
    fn count_swars(&self) -> usize {
        self.0.len()
    }

    fn n_swars(&self, n: usize) -> Option<SwarBlock> {
        let _swars: Vec<Swar> = (&self.0).clone();
        let swars = &_swars[0..n];
        Some(SwarBlock(Vec::from(swars)))
    }
}

impl Melody for SwarBlock {
    fn play(&self, dev: &AudioDevice, beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>, n: i8) {
        for _ in 0..n {
            for bt in &self.0 {
                print!("{} ", bt);
                utils::io_flush();
                play_swar_with_taal(&dev, &bt, Some(&beat_src));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lineA: Option<Vec<SwarBlock>>,
    pub lineB: Option<Vec<SwarBlock>>,
    pub lineC: Option<Vec<SwarBlock>>,
}

impl Sthayi {
    pub fn new(lineA: Option<Vec<SwarBlock>>, lineB: Option<Vec<SwarBlock>>, lineC: Option<Vec<SwarBlock>>) -> Self {
        Sthayi {
            lineA,
            lineB,
            lineC
        }
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub lineC: Option<Vec<SwarBlock>>,
    pub lineD: Option<Vec<SwarBlock>>,
    pub lineE: Option<Vec<SwarBlock>>,
}

impl Antara {
    pub fn new(lineC: Option<Vec<SwarBlock>>, lineD: Option<Vec<SwarBlock>>, lineE: Option<Vec<SwarBlock>>) -> Self {
       Antara  {
            lineC,
            lineD,
            lineE
        }
    }
}

#[derive(Debug, Clone)]
pub struct Swarmaalika {
    pub mukra: Option<Vec<SwarBlock>>,
    pub sthayi: Sthayi,
    pub antara: Antara,
    pub tihayi: Option<Vec<SwarBlock>>,
}

impl Swarmaalika {
    pub fn new(mukra: Option<Vec<SwarBlock>>,
               sthayi: Sthayi,
               antara: Antara,
               tihayi: Option<Vec<SwarBlock>>) -> Self {
        Swarmaalika { mukra, sthayi, antara, tihayi }
    }
}

impl Melody for Swarmaalika {
    // TODO: should the beat_src be a reference (&beat_src)?
    // [mukra] <sthayi> A <antara> A <tihayi> X 3
    // [mukra] <A A B B [C]> A <C C D D E E] A <tihayi> X 3
    fn play(&self, dev: &AudioDevice, beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>, n: i8) {
        // play: sthayi, line A of sthayi, antara, line A of sthayi, tihayi
        println!("\nPlaying swarmaalika");
        let sthayi = &self.sthayi;
        let antara = &self.antara;

        match &self.mukra {
            Some(line) => {
                for blk in line{
                    blk.play(&dev, beat_src.clone(), 1);
                }
            },
            _ => {}
        }

        // TODO: closure for iteration
        match &sthayi.lineA {
            Some(line) => {
                for  _ in 0..2 {
                    for blk in line {
                        blk.play(&dev, beat_src.clone(), 1);
                    }
                    println!();
                }
            },
            _ => {}
        }

        match &sthayi.lineB {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 2);
                    println!();
                }
            },
            _ => {}
        }

        //  if line C exists, play it once
        match &sthayi.lineC {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 2);
                    println!();
                }
            },
            _ => {}
        }

        match &sthayi.lineA {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 1);
                    println!();
                }
            },
            _ => {}
        }

        match &antara.lineC {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 2);
                    println!();
                }
            },
            _ => {}
        }

        match &antara.lineD {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 2);
                    println!();
                }
            },
            _ => {}
        }

        match &antara.lineE {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 2);
                    println!();
                }
            },
            _ => {}
        }

        match &sthayi.lineA {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 1);
                    println!();
                }
            },
            _ => {}
        }

        match &self.tihayi {
            Some(line) => {
                for blk in line{
                    blk.play(&dev, beat_src.clone(), TIHAYI_TIMES);
                }
            },
            _ => {}
        }
        println!();
    }
}

#[derive(Debug, Clone)]
pub struct Raag {
    name: String,
    aroha: Option<Vec<SwarBlock>>,
    avroha: Option<Vec<SwarBlock>>,
    pakad: Option<Vec<SwarBlock>>,
    alankars: Option<Vec<SwarBlock>>,
    swarmaalika: Swarmaalika,
}

impl Raag {
    pub fn new(
        name: String,
        aroha: Option<Vec<SwarBlock>>,
        avroha: Option<Vec<SwarBlock>>,
        pakad: Option<Vec<SwarBlock>>,
        alankars: Option<Vec<SwarBlock>>,
        swarmaalika: Swarmaalika,
    ) -> Raag {
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

    pub fn aroha(&self) -> &Option<Vec<SwarBlock>> {
        &self.aroha
    }

    pub fn avroha(&self) -> &Option<Vec<SwarBlock>> {
        &self.avroha
    }

    pub fn pakad(&self) -> &Option<Vec<SwarBlock>> {
        &self.pakad
    }

    pub fn alankars(&self) -> &Option<Vec<SwarBlock>> {
        &self.alankars
    }

    pub fn swarmaalika(&self) -> &Swarmaalika {
        &self.swarmaalika
    }

    fn play_aroha(&self,
                  dev: &AudioDevice,
                  beat_src: &Repeat<TakeDuration<Decoder<BufReader<File>>>>) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        match self.aroha() {
            Some(_aroha) => {
                for blk in _aroha {
                    blk.play(&dev, (*beat_src).clone(), 1);
                }
            },
            _ => {}
        }
    }

    fn play_avroha(&self, dev: &AudioDevice, beat_src: &Repeat<TakeDuration<Decoder<BufReader<File>>>>) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        match self.avroha() {
            Some(_avroha) => {
                for blk in _avroha {
                    blk.play(&dev, (*beat_src).clone(), 1);
                }
            },
            _ => {}
        }
    }

    fn play_pakad(&self, dev: &AudioDevice, beat_src: &Repeat<TakeDuration<Decoder<BufReader<File>>>>) {
        println!("\n=> Playing pakad for raag: {}", self.name());
        match self.pakad() {
            Some(_pakad) => {
                let mut _comma: bool = false;
                for blk in _pakad {
                    if _comma {
                        print!(", ");
                        utils::io_flush();
                    }
                    _comma = true;
                    blk.play(&dev, (*beat_src).clone(), 1);
                }
            },
            _ => {}
        }
    }

    #[allow(dead_code)]
    fn play_alankars(
        &self,
        dev: &AudioDevice,
        beat_src: &Repeat<TakeDuration<Decoder<BufReader<File>>>>,
    ) {
        println!("\n=> Playing alankars for raag: {}", self.name());
        match self.alankars() {
            Some(_alankar) => {
                let mut _comma: bool = false;
                for blk in _alankar {
                    if _comma {
                        print!(", ");
                        utils::io_flush();
                    }
                    blk.play(&dev, (*beat_src).clone(), 1);
                }
            },
            _ => {}
        }
    }
}

impl Melody for Raag {
    fn play(&self, dev: &AudioDevice, beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>, n: i8) {
        let gap: f32 = 1.0; //no of beats
        self.play_aroha(&dev, &beat_src);
        utils::delay(gap * BPS);
        self.play_avroha(&dev, &beat_src);
        utils::delay(gap * BPS);
        self.play_pakad(&dev, &beat_src);
        utils::delay(gap * BPS);
        self.swarmaalika.play(dev, beat_src.clone(), n);
        utils::delay(gap * BPS);
        self.play_alankars(&dev, &beat_src);
    }
}

pub fn play_swar_with_taal(
    dev: &AudioDevice,
    sw: &Swar,
    beat_source: Option<&Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
) {
    let sink = Sink::new(&dev.dev);
    match &sw.pitch {
        Some(p) => {
            let sinew = SineWave::from(p.to_owned());
            match beat_source {
                Some(src) => {
                    let _bt_src = src.clone();
                    sink.append(sinew.mix(_bt_src));
                }
                _ => {
                    sink.append(sinew);
                }
            };
            sink.set_volume(*&dev.vol as f32);
            sink.play();
            utils::delay(sw.beat_cnt * BPS);
            sink.stop();
        }
        _ => {}
    }
}

pub fn play_swar(dev: &AudioDevice, sw: &Swar) {
    let sink = Sink::new(&dev.dev);
    match &sw.pitch {
        Some(p) => {
            // let mixr =  mixer(1, 1); //DynamicMixerController::add(beep);
            // mixr.0.add(beep);
            // mixr.0.add(sinew);
            let sinew = SineWave::from(p.to_owned());
            sink.append(sinew);
            sink.set_volume(*&dev.vol as f32);
            sink.play();
            utils::delay(sw.beat_cnt * BPS);
            sink.stop();
        }
        _ => {}
    }
}
