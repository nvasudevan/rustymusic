use rand;
use rand::seq::SliceRandom;
use rodio::source::{Repeat, TakeDuration, SineWave};
use rodio::decoder::Decoder;
use rodio::{Sink, Source, PlayError};
use std::fs::File;
use std::io::BufReader;
use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::swars::{SwarBlock, Swar, BeatSrc, SwarBlocks};
use crate::raagas::sound::{AudioDevice, TimedSink};
use crate::raagas::constants::{PLAY_PAUSE_DURATION, BPS};
use crate::raagas::utils;
use crate::raagas::raag::SimpleRandomiser;

use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use crate::raagas::raag::random::index_swar;

#[derive(Clone)]
pub struct Raag {
    swarmaalika: Swarmaalika,
    name: String,
    aroha: Option<SwarBlocks>,
    avroha: Option<SwarBlocks>,
    pakad: Option<SwarBlocks>,
    alankars: Option<SwarBlocks>,
    beat_src: Option<BeatSrc>,
}

impl Raag {
    pub fn new(
        name: String,
        aroha: Option<SwarBlocks>,
        avroha: Option<SwarBlocks>,
        pakad: Option<SwarBlocks>,
        alankars: Option<SwarBlocks>,
        swarmaalika: Swarmaalika,
        beat_src: Option<BeatSrc>,
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

    pub fn aroha(&self) -> &Option<SwarBlocks> {
        &self.aroha
    }

    pub fn avroha(&self) -> &Option<SwarBlocks> {
        &self.avroha
    }

    pub fn pakad(&self) -> &Option<SwarBlocks> {
        &self.pakad
    }

    pub fn alankars(&self) -> &Option<SwarBlocks> {
        &self.alankars
    }

    pub fn swarmaalika(&self) -> &Swarmaalika {
        &self.swarmaalika
    }

    pub fn beat_src(&self) -> &Option<BeatSrc> {
        &self.beat_src
    }

    fn build_aroha(&self, dev: &AudioDevice, vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        println!("\n=> aroha for raag: {}", self.name());
        self.aroha.as_ref().unwrap().build_sink(&None, &dev, vol)
    }

    fn build_avroha(&self, dev: &AudioDevice, vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        println!("\n=> avroha for raag: {}", self.name());
        self.avroha.as_ref().unwrap().build_sink(&None, &dev, vol)
    }

    fn build_pakad(&self, dev: &AudioDevice, vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        println!("\n=> pakad for raag: {}", self.name());
        self.pakad.as_ref().unwrap().build_sink(&None, &dev, vol)
    }

    fn build_alankars(&self, dev: &AudioDevice, vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        println!("\n=> alankars for raag: {}", self.name());
        self.alankars.as_ref().unwrap().build_sink(&self.beat_src, &dev, vol)
    }

    fn build_swarmaalika(&self, dev: &AudioDevice, vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        self.swarmaalika.build_sink(&self.beat_src, &dev, vol)
    }

    pub fn is_ascending(&self, swars: &Vec<Swar>) -> bool {
        let aroha = self.aroha().as_ref().unwrap().to_swars();

        let mut i = index_swar(&aroha, swars.first().unwrap());
        if let Some(swars_tail) = swars.get(1..) {
            for swar in swars_tail {
                let _i = index_swar(&aroha, swar);
                if _i < i {
                    return false
                }
                i = _i;
            }
        }

        return true;
    }

    pub fn is_descending(&self, swars: &Vec<Swar>) -> bool {
        let avroha = self.avroha().as_ref().unwrap().to_swars();

         if let Some(mut i) = index_swar(&avroha, swars.first().unwrap()) {
             if let Some(swars_tail) = swars.get(1..) {
                 for swar in swars_tail {
                     if let Some(_i) = index_swar(&avroha, swar) {
                         if _i < i {
                             return false
                         }
                         i = _i;
                     } else {
                         // if swar not in avroha, return false
                         return false;
                     }
                 }
             }
             return true; //not sure about this
         }

        // the initial swar not in avroha
        return false;
    }

    pub fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        _beat_src: Option<BeatSrc>,
        _mix: bool,
        n: i8,
    ) {
        println!("=> build sink: {}", self.name());
        println!("=> playing raag: {}", self.name());
        let play_sinks = |sinks: Result<Vec<TimedSink>, PlayError>| match &sinks {
            Ok(timed_sinks) => {
                println!("=> playing ...");
                for tsink in timed_sinks {
                    tsink.sink.set_volume(1.0);
                    tsink.sink.play();
                    utils::delay(tsink.duration * BPS);
                    tsink.sink.stop();
                    println!("sink => len: {}, empty: {}", tsink.sink.len(), tsink.sink.empty());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
        // play_sinks(self.build_aroha(&dev, vol));
        // utils::delay(PLAY_PAUSE_DURATION * BPS);
        // play_sinks(self.build_avroha(&dev, vol));
        // utils::delay(PLAY_PAUSE_DURATION  * BPS);
        // play_sinks(self.build_pakad(&dev, vol));
        // utils::delay(PLAY_PAUSE_DURATION * BPS);
        play_sinks(self.build_swarmaalika(&dev, vol));
        // utils::delay(PLAY_PAUSE_DURATION * BPS);
        // self.play_alankars(&dev, &beat_src);
        // utils::delay(2.0);
    }
}

