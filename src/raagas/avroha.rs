use crate::raagas::swars::Swar;
use crate::raagas::swarblocks::SwarBlocks;
use crate::raagas::sound::AudioDevice;
use crate::raagas::swarblock::SwarBlock;

/// Avroha in all three octaves
#[derive(Clone)]
pub struct Avroha {
    avroha: SwarBlocks,
    lower: Option<SwarBlocks>,
    higher: Option<SwarBlocks>,
}

impl Avroha {
    /// Returns an Avroha along with the lower and the higher
    /// octave equivalent
    pub fn new(avroha: SwarBlocks) -> Self {
        Avroha {
            avroha,
            lower: None,
            higher: None
        }
    }

    /// Build avroha in lower octave
    /// for both lower and higher octaves, lose the first and last swarbeat
    /// e.g.: for S. D P M R S, lower is .D .P .M .R , higher is D. P. M. R.
    /// full sequence: D. P. M. R. | S. D P M R S | .D .P .M .R
    /// To simplify, aroha/avroha only have:
    ///   - one or two `swarblock`,
    ///   - have always one swar per swarbeat
    pub fn build(&self, blks: &Vec<SwarBlock>) -> SwarBlocks {
        let mut octave_blks = Vec::<SwarBlock>::new();
        let first_blk = blks.first().unwrap();

        // in aroha the first swar beat has always a swar, so we can safely
        // start from 1st beat
        let start = first_blk.next_index_with_swar(
            1,
            first_blk.len(),
            true).unwrap();

        // if there is a second block, lose the swarbeat from it
        // if not lose the last swarbeat from the first block
        return match blks.get(1) {
            Some(second_blk) => {
                // lose the last swarbeat
                // let second_blk_beats = second_blk.0.get(0..(second_blk.len() - 1)).unwrap();
                // let second_blk_beats_vec = second_blk_beats.to_vec();
                let end = second_blk.next_index_with_swar(
                    0,
                    second_blk.len()-1,
                    false).unwrap();

                let first_blk_beats_vec = first_blk.0.get(start..).unwrap().to_vec();
                let second_blk_beats_vec = second_blk.0.get(0..end).unwrap().to_vec();
                octave_blks.push(SwarBlock(first_blk_beats_vec));
                octave_blks.push(SwarBlock(second_blk_beats_vec));

                SwarBlocks(octave_blks)
            },
            _ => {
                // find from the last swarbeat, index which has a swar
                let end = first_blk.next_index_with_swar(
                    0,
                    first_blk.len()-1,
                    false).unwrap();
                let beats_vec = first_blk.0.get(start..end).unwrap().to_vec();
                octave_blks.push(SwarBlock(beats_vec));
                SwarBlocks(octave_blks)
            }
        }
    }

    pub fn build_lower(&mut self) {
        let blks = self.build(&self.avroha.0);
        self.lower = Some(blks.lower());
    }

    pub fn build_higher(&mut self) {
        let blks = self.build(&self.avroha.0);
        self.higher = Some(blks.higher());
    }

    pub fn lower(&self) -> Option<&SwarBlocks> {
        self.lower.as_ref()
    }

    pub fn higher(&self) -> Option<&SwarBlocks> {
        self.higher.as_ref()
    }

    pub fn avroha(&self) -> &SwarBlocks {
        &self.avroha
    }

    /// Returns all three octaves of avroha
    pub fn all_octaves(&self) -> SwarBlocks {
        let mut blks = Vec::<SwarBlock>::new();
        let mut higher_blks = self.higher().unwrap().0
            .iter()
            .map(|blk| blk.clone())
            .collect::<Vec<SwarBlock>>();
        blks.append(&mut higher_blks);

        let mut avroha_blks = self.avroha().0
            .iter()
            .map(|blk| blk.clone())
            .collect::<Vec<SwarBlock>>();
        blks.append(&mut avroha_blks);

        let mut lower_blks = self.lower().unwrap().0
            .iter()
            .map(|blk| blk.clone())
            .collect::<Vec<SwarBlock>>();
        blks.append(&mut lower_blks);

        SwarBlocks(blks)
    }
    /// Returns avroha swars in all three octaves (in order):
    /// higher middle lower
    /// e.g.: For swars x y z, return x. y. z. x y z .x .y .z
    pub fn swars_in_all_octaves(&self) -> Vec<&Swar> {
        let mut swars = Vec::<&Swar>::new();
        for sw in self.higher.as_ref().unwrap().to_swars_as_ref() {
            swars.push(sw);
        }

        swars.extend(self.avroha.to_swars_as_ref());

        for sw in self.lower.as_ref().unwrap().to_swars_as_ref() {
            swars.push(sw);
        }

        swars
    }

    pub fn swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let all_swars = [
            self.higher.as_ref().unwrap(),
            &self.avroha,
            &self.lower.as_ref().unwrap()
        ];

        for blks in all_swars.iter() {
            if let Some(index) = blks.index_swar(&swar) {
                return blks.adjacent_swars(&index);
            }
        }

        None
    }

    pub fn play(&self, dev: &AudioDevice) {
        self.avroha.play(&dev);
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;
    use crate::raagas::avroha::Avroha;

    /// test avroha as string
    #[test]
    fn test_avroha() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "S. - D - P - M - R - S -";

        let avroha = raag.avroha();
        assert_eq!(avroha.avroha().to_string(), expected);
    }

    /// test avroha lower octave
    #[test]
    fn test_avroha_lower_octave() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = ".D - .P - .M - .R -";
        let avroha = raag.avroha().build(&raag.avroha().avroha.0);

        assert_eq!(avroha.lower().to_string(), expected);
    }

    /// test avvroha higher octave
    #[test]
    fn test_avroha_higher_octave() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "D. - P. - M. - R. -";
        let avroha = raag.avroha().build(&raag.avroha().avroha.0);

        assert_eq!(avroha.higher().to_string(), expected);
    }

    /// test avroha in all three octaves
    #[test]
    fn test_avroha_all_octaves() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "D. - P. - M. - R. - S. - D - P - M - R - S - .D - .P - .M - .R -";

        let mut avroha = Avroha::new(raag.avroha().avroha.clone());
        avroha.build_lower();
        avroha.build_higher();

        let avroha_set = avroha.all_octaves();
        assert_eq!(avroha_set.to_string(), expected);
    }
}