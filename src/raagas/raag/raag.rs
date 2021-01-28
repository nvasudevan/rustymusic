use rodio::{PlayError};

use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::sound::{AudioDevice, TimedSink};
use crate::raagas::constants::{BPS};
use crate::raagas::utils;
use crate::raagas::raag::random::index_swar;
use crate::raagas::swarblock::SwarBlocks;

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

    pub fn asc_desc(&self, base_swars: Vec<Swar>, swars: &Vec<Swar>) -> bool {
        if let Some(mut i) = index_swar(&base_swars, swars.first().unwrap()) {
            if let Some(swars_tail) = swars.get(1..) {
                for swar in swars_tail {
                    if let Some(_i) = index_swar(&base_swars, swar) {
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

    pub fn is_ascending(&self, swars: &Vec<Swar>) -> bool {
        self.asc_desc(self.aroha().as_ref().unwrap().to_swars(), swars)

    }
    pub fn is_descending(&self, swars: &Vec<Swar>) -> bool {
        self.asc_desc(self.avroha().as_ref().unwrap().to_swars(), swars)
    }

    pub fn aroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<Swar>> {
        let swars = self.aroha.as_ref().unwrap().to_swarblock();
        if let Some(i) = index_swar(&swars.to_swars(), &swar) {
            return swars.adjacent_swars(i);
        }

        None
    }

    pub fn avroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<Swar>> {
        let swars = self.avroha.as_ref().unwrap().to_swarblock();
        if let Some(i) = index_swar(&swars.to_swars(), &swar) {
            return swars.adjacent_swars(i);
        }

        None
    }

    pub fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        _beat_src: Option<BeatSrc>,
        _mix: bool,
        _n: i8,
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

