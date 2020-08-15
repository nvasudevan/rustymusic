use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::elements::swarmaalika::Swarmaalika;
use rodio::source::{Repeat, TakeDuration};
use rodio::decoder::Decoder;
use std::io::BufReader;
use std::fs::File;
use crate::raagas::elements::elements::{AudioDevice, Melody, BPS};
use crate::raagas::utils;

#[derive(Debug, Clone)]
pub struct Raag {
    swarmaalika: Swarmaalika,
    name: String,
    aroha: Option<Vec<SwarBlock>>,
    avroha: Option<Vec<SwarBlock>>,
    pakad: Option<Vec<SwarBlock>>,
    alankars: Option<Vec<SwarBlock>>,
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

