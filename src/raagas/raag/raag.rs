use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::swars::{SwarBlock, Melody};
use crate::raagas::physics::AudioDevice;
use crate::raagas::constants::{PLAY_PAUSE_DURATION, BPS};
use crate::raagas::utils;

#[derive(Clone)]
pub struct Raag {
    swarmaalika: Swarmaalika,
    name: String,
    aroha: Option<Vec<SwarBlock>>,
    avroha: Option<Vec<SwarBlock>>,
    pakad: Option<Vec<SwarBlock>>,
    alankars: Option<Vec<SwarBlock>>,
    beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
}

impl Raag {
    pub fn new(
        name: String,
        aroha: Option<Vec<SwarBlock>>,
        avroha: Option<Vec<SwarBlock>>,
        pakad: Option<Vec<SwarBlock>>,
        alankars: Option<Vec<SwarBlock>>,
        swarmaalika: Swarmaalika,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
    ) -> Raag {
        Raag {
            name,
            aroha,
            avroha,
            pakad,
            alankars,
            swarmaalika,
            beat_src,
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

    pub fn beat_src(&self) -> &Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>> {
        &self.beat_src
    }

    fn play_aroha(&self, dev: &AudioDevice, vol: f32) {
        println!("\n=> aroha for raag: {}", self.name());
        match self.aroha() {
            Some(_aroha) => {
                for blk in _aroha {
                    blk.play(&dev, vol, None, false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_avroha(&self, dev: &AudioDevice, vol: f32) {
        println!("\n=> avroha for raag: {}", self.name());
        match self.avroha() {
            Some(_avroha) => {
                for blk in _avroha {
                    blk.play(&dev, vol, None, false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_pakad(&self, dev: &AudioDevice, vol: f32) {
        println!("\n=> pakad for raag: {}", self.name());
        match self.pakad() {
            Some(_pakad) => {
                let mut _comma: bool = false;
                for blk in _pakad {
                    if _comma {
                        print!(", ");
                        utils::io_flush();
                    }
                    _comma = true;
                    blk.play(&dev, vol,None, false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_alankars(&self, dev: &AudioDevice, vol: f32) {
        println!("\n=> alankars for raag: {}", self.name());
        match self.alankars() {
            Some(_alankar) => {
                let mut _comma: bool = false;
                for blk in _alankar {
                    if _comma {
                        print!(", ");
                        utils::io_flush();
                    }
                    blk.play(&dev, vol, self.beat_src.clone(), false, 1);
                }
            }
            _ => {}
        }
    }

    fn play_swarmaalika(&self, dev: &AudioDevice, vol: f32, mix: bool, n: i8) {
        self.swarmaalika.play(dev, vol, self.beat_src.clone(), mix, n);
    }
}

impl Melody for Raag {
    fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        _beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        _mix: bool,
        n: i8,
    ) {
        println!("=> playing raag: {}", self.name());
        match Sink::try_new(&dev.out_stream_handle) {
            Ok(sink) => {
                self.play_aroha(&dev, vol);
                utils::delay(PLAY_PAUSE_DURATION * BPS);
                self.play_avroha(&dev, vol);
                utils::delay(PLAY_PAUSE_DURATION  * BPS);
                self.play_pakad(&dev, vol);
                utils::delay(PLAY_PAUSE_DURATION * BPS);
                // self.play_taal(&sink, beat_src);
                // sink.set_volume(*&dev.vol as f32);
                self.play_swarmaalika(dev, vol, false, n);
                utils::delay(PLAY_PAUSE_DURATION * BPS);
                // self.play_alankars(&dev, &beat_src);
                sink.play();
                utils::delay(2.0);
                // sink.stop();
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
