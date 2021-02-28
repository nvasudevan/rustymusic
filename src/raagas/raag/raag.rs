use crate::raagas::swarmaalika::Swarmaalika;
use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::sound::{AudioDevice};
use crate::raagas::constants::{BPS, PLAY_PAUSE_DURATION};
use crate::raagas::utils;
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

    fn play_aroha(&self, dev: &AudioDevice) {
        println!("\n=> playing aroha  {}", self.aroha.as_ref().unwrap());
        self.aroha.as_ref().unwrap().play(&dev);
    }

    fn play_avroha(&self, dev: &AudioDevice) {
        println!("\n=> playing avroha  {}", self.avroha.as_ref().unwrap());
        self.avroha.as_ref().unwrap().play(&dev);
    }

    fn play_pakad(&self, dev: &AudioDevice) {
        // println!("\n=> playing pakad  {:?}", self.pakad.unwrap());
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

    pub fn is_ascending(&self, swars: &Vec<&Swar>) -> bool {
        self.aroha().as_ref().unwrap().is_monotonic_increasing(swars)
    }

    pub fn is_descending(&self, swars: &Vec<&Swar>) -> bool {
        self.avroha().as_ref().unwrap().is_monotonic_increasing(swars)
    }

    pub fn aroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let blks = self.aroha.as_ref().unwrap();
        if let Some(index) = blks.index_swar(&swar) {
            return blks.adjacent_swars(&index);
        }

        None
    }

    pub fn avroha_swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let blks = self.avroha.as_ref().unwrap();
        if let Some(index) = blks.index_swar(&swar) {
            return blks.adjacent_swars(&index);
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

    /// test if raag durga composition can be loaded and if it contains aroha
    #[test]
    fn test_load_raag_durga_aroha_as_string() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "S - R - M - P - D - S. - -";

        let aroha =  raag.aroha().as_ref().unwrap();
        assert_eq!(aroha.to_string(), expected);
    }

    /// test if raag composition can be loaded and contains parts:
    /// aroha, avroha, pakad, alankars, sthayi, antara
    #[test]
    fn test_load_raag_parts() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();

        assert!(raag.aroha().is_some());
        assert!(raag.avroha().is_some());
        assert!(raag.pakad().is_some());
        assert!(raag.alankars().is_some());
        assert_eq!(raag.swarmaalika().sam(), 1);
        assert!(!raag.swarmaalika().sthayi.lines.is_empty());
        assert!(!raag.swarmaalika().antara.lines.is_empty());
    }
}