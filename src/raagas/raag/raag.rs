use rodio::{PlayError};

use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::sound::{AudioDevice, TimedSink};
use crate::raagas::constants::{BPS, PLAY_PAUSE_DURATION};
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

    fn play_aroha(&self, dev: &AudioDevice, vol: f32) {
        let blk = self.aroha.as_ref().unwrap().to_swarblock();
        println!("\n=> aroha  {}", blk);
        blk.play(&dev, vol);
    }

    fn play_avroha(&self, dev: &AudioDevice, vol: f32) {
        let blk = self.avroha.as_ref().unwrap().to_swarblock();
        println!("\n=> avroha  {}", blk);
        blk.play(&dev, vol);
    }

    fn play_pakad(&self, dev: &AudioDevice, vol: f32) {
        let blk = self.pakad.as_ref().unwrap().to_swarblock();
        println!("\n=> pakad  {}", blk);
        blk.play(&dev, vol);
    }

    fn play_alankars(&self, dev: &AudioDevice, vol: f32) {
        let blk = self.alankars.as_ref().unwrap().to_swarblock();
        println!("\n=> alankars  {}", blk);
        blk.play(&dev, vol);
    }

    fn play_swarmaalika(&self, dev: &AudioDevice, vol: f32)  {
        println!("\n=> playing swarmaalika");
        self.swarmaalika.play(&dev, vol);
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

    pub fn play(&self, dev: &AudioDevice, vol: f32) {
        println!("=> playing raag: {}", self.name());

        self.play_aroha(&dev, vol);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_avroha(&dev, vol);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_pakad(&dev, vol);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_swarmaalika(&dev, vol);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
    }
}
