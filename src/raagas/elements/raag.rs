use crate::raagas::elements::elements::{AudioDevice, Melody, Taal, BPS};
use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::elements::swarmaalika::Swarmaalika;
use crate::raagas::utils;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;

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

    fn play_aroha(
        &self,
        dev: &AudioDevice,
        beat_src: &Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
    ) {
        println!("\n=> Playing aroha for raag: {}", self.name());
        match self.aroha() {
            Some(_aroha) => {
                for blk in _aroha {
                    blk.play(&dev, (*beat_src).clone(), false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_avroha(
        &self,
        dev: &AudioDevice,
        beat_src: &Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
    ) {
        println!("\n=> Playing avroha for raag: {}", self.name());
        match self.avroha() {
            Some(_avroha) => {
                for blk in _avroha {
                    blk.play(&dev, (*beat_src).clone(), false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_pakad(
        &self,
        dev: &AudioDevice,
        beat_src: &Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
    ) {
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
                    blk.play(&dev, (*beat_src).clone(),  false,1);
                }
            }
            _ => {}
        }
    }

    fn play_alankars(
        &self,
        dev: &AudioDevice,
        beat_src: &Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
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
                    blk.play(&dev, (*beat_src).clone(), false, 1);
                }
            }
            _ => {}
        }
    }
}

impl Melody for Raag {
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        mix: bool,
        n: i8,
    ) {
        let gap: f32 = 1.0; //no of beats
        let bt = None;
        let sink = Sink::new(&dev.dev);
        self.play_aroha(&dev, &bt);
        utils::delay(gap * BPS);
        // self.play_avroha(&dev, &bt);
        // utils::delay(gap * BPS);
        self.play_pakad(&dev, &bt);
        utils::delay(gap * BPS);
        // self.play_taal(&sink, beat_src);
        //sink.set_volume(*&dev.vol as f32);
        self.swarmaalika.play(dev, beat_src.clone(), false, n);
        // utils::delay(gap * BPS);
        // self.play_alankars(&dev, &beat_src);
        sink.play();
        utils::delay(2.0);
        sink.stop();
    }
}
