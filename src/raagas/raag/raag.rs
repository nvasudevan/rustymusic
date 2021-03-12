use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::sound::{AudioDevice};
use crate::raagas::constants::{BPS, PLAY_PAUSE_DURATION};
use crate::raagas::{utils, swars};
use crate::raagas::swarblocks::SwarBlocks;
use crate::raagas::swars::Swar;
use crate::raagas::aroha_avroha::{Aroha, Avroha};

#[derive(Clone)]
pub struct Raag {
    swarmaalika: Swarmaalika,
    name: String,
    aroha: Aroha,
    avroha: Avroha,
    pakad: Option<SwarBlocks>,
    alankars: Option<SwarBlocks>,
    beat_src: Option<swars::BeatSrc>,
}

impl Raag {
    pub fn new(
        name: String,
        aroha: Aroha,
        avroha: Avroha,
        pakad: Option<SwarBlocks>,
        alankars: Option<SwarBlocks>,
        swarmaalika: Swarmaalika,
        beat_src: Option<swars::BeatSrc>,
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

    pub fn aroha(&self) -> &Aroha {
        &self.aroha
    }

    pub fn avroha(&self) -> &Avroha {
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

    pub fn beat_src(&self) -> &Option<swars::BeatSrc> {
        &self.beat_src
    }

    fn play_aroha(&self, dev: &AudioDevice) {
        println!("\n=> playing aroha  {}", self.aroha.aroha());
        self.aroha.play(&dev);
    }

    fn play_avroha(&self, dev: &AudioDevice) {
        println!("\n=> playing avroha  {}", self.avroha.avroha());
        self.avroha.play(&dev);
    }

    fn play_pakad(&self, dev: &AudioDevice) {
        println!("\n=> playing pakad  {}", self.pakad.as_ref().unwrap());
        self.pakad.as_ref().unwrap().play(&dev);
    }

    // fn play_alankars(&self, dev: &AudioDevice, vol: f32) {
    //     // println!("\n=> playing alankars  {:?}", self.alankars.unwrap());
    //     self.alankars.as_ref().unwrap().play(&dev, vol);
    // }

    fn play_swarmaalika(&self, dev: &AudioDevice)  {
        println!("\n=> playing swarmaalika");
        self.swarmaalika.play(&dev);
    }

    /// check if the `swars` are in aroha in all three octaves
    pub fn in_aroha(&self, swars: &Vec<&Swar>) -> bool {
        swars::contains(&self.aroha.swars_in_all_octaves(), &swars)
    }

    /// check if the `swars` are in avroha in all three octaves
    pub fn in_avroha(&self, swars: &Vec<&Swar>) -> bool {
        swars::contains(&self.avroha.swars_in_all_octaves(), &swars)
    }

    pub fn aroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let all_blks = self.aroha.all_octaves();
        for blks in all_blks {
            if let Some(index) = blks.index_swar(&swar) {
                return blks.adjacent_swars(&index);
            }
        }

        None
    }

    pub fn avroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let all_blks = self.avroha.all_octaves();
        for blks in all_blks {
            if let Some(index) = blks.index_swar(&swar) {
                return blks.adjacent_swars(&index);
            }
        }

        None
    }

    pub fn play(&self, dev: &AudioDevice) {
        println!("=> playing raag: {}", self.name());

        self.play_aroha(&dev);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_avroha(&dev);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_pakad(&dev);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
        self.play_swarmaalika(&dev);
        utils::delay(PLAY_PAUSE_DURATION * BPS);
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;

    /// test if raag composition can be loaded and contains parts:
    /// aroha, avroha, pakad, alankars, sthayi, antara
    #[test]
    fn test_load_raag_parts() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();

        assert!(raag.aroha().aroha().0.len() > 0);
        assert!(raag.avroha().avroha().0.len() > 0);
        assert!(raag.pakad().is_some());
        assert!(raag.alankars().is_some());
        assert_eq!(raag.swarmaalika().sam(), 1);
        assert!(!raag.swarmaalika().sthayi.lines.is_empty());
        assert!(!raag.swarmaalika().antara.lines.is_empty());
    }
}