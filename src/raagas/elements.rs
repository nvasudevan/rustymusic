use crate::raagas::{swars, bhupali};
use std::fmt;
use std::time;
use std::thread::sleep;
use std::time::Duration;
use rodio::{Sink, source::SineWave, default_output_device, output_devices, Device};
use crate::raagas::swars::{Pitch, BASE_SWAR_INTERVAL};
use std::io::{Read, Write, BufReader};
use std::fmt::Formatter;
use std::error::Error;
use rodio::source::Buffered;
use crate::BEEP;

#[derive(Debug, Clone)]
pub struct Beat<'a> {
    pub swar: Option<Pitch<'a>>,
    pub long: u64,
}

impl<'a> fmt::Display for Beat<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dash = (0..(self.long-1)).map(|_| "-").collect::<String>();
        let mut _s = ".".to_string();
        match &self.swar {
            Some(sw) => {
                _s  = format!("{}{}", self.swar.unwrap(), dash);
            },
            _ => {}
        }
        write!(f, "{}", _s)
    }
}

#[derive(Debug, Clone)]
pub struct SwarBlock<'a>(pub Vec<Beat<'a>>);

#[derive(Debug, Clone)]
pub struct Raag<'a> {
    name: String,
    aroha: Vec<Beat<'a>>,
    avroha: Vec<Beat<'a>>,
    pakad: Vec<SwarBlock<'a>>
}

impl<'a> Raag<'a> {
    pub fn new(name: String, aroha: Vec<Beat<'a>>, avroha: Vec<Beat<'a>>, pakad: Vec<SwarBlock<'a>>) -> Raag<'a> {
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
}

pub trait Melody {
    fn play_aroha(&self, dev: &Device);
    fn play_avroha(&self, dev: &Device);
    fn play_pakad(&self, dev: &Device);
    fn play(&self, dev: &Device) {
        self.play_aroha(dev);
        delay(2);
        self.play_avroha(&dev);
        delay(2);
        self.play_pakad(&dev);
    }
}

impl<'a> Melody for Raag<'a> {
     fn play_aroha(&self, dev: &Device) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        for bt in self.aroha() {
            print!("{} ", bt);
            std::io::stdout().flush();
            play_swar(&dev, &bt);
        }
    }

    fn play_avroha(&self, dev: &Device) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        for bt in self.avroha() {
            print!("{} ", bt.swar.unwrap());
            std::io::stdout().flush();
            play_swar(&dev, &bt);
        }
    }

    fn play_pakad(&self, dev: &Device) {
        println!("\n=> Playing pakad for raag: {}", self.name());
        let mut _comma: bool = false;
        for blk in self.pakad() {
            if _comma {
                print!(", ");
                std::io::stdout().flush();
            }
            _comma = true;
            for bt in &blk.0 {
                print!("{} ", bt);
                std::io::stdout().flush();
                play_swar(&dev, &bt);
            }
        }
    }
}

pub fn delay(t: u64) {
    sleep(Duration::from_secs(t));
}

pub fn play_swar(dev: &Device, bt: &Beat) {
    let sink = Sink::new(&dev);
    match &bt.swar {
        Some(p) =>  {
            let sw = SineWave::from(*p);
            sink.append(sw);
            sink.play();
            delay(bt.long);
            sink.stop();
        },
        _ => {
            let f: std::fs::File = std::fs::File::open("./samples/beep.wav").unwrap();
            let beep = rodio::play_once(&dev, BufReader::new(f)).unwrap();
            beep.set_volume(0.2);
            delay(swars::BASE_SWAR_INTERVAL);
            beep.stop();
        }
    }
}
