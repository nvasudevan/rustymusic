use crate::raagas::swars::Swar;
use crate::raagas::swarblocks::SwarBlocks;
use crate::raagas::sound::AudioDevice;
use crate::raagas::swarblock::SwarBlock;

/// Aroha in all three octaves
#[derive(Clone)]
pub struct Aroha {
    aroha: SwarBlocks,
    lower: Option<SwarBlocks>,
    higher: Option<SwarBlocks>
}

impl Aroha {
    pub fn new(aroha: SwarBlocks) -> Self {
        Aroha {
            aroha,
            lower: None,
            higher: None
        }
    }

    /// Build aroha in lower octave
    /// for both lower and higher octaves, lose the first and last swarbeat
    /// e.g.: for S R M P D S., lower is .R .M .P .D, higher is R. M. P. D.
    /// full sequence: .R .M .P .D | S R M P D S. | R. M. P. D.
    /// To simplify, aroha/avroha only have:
    ///   - one or two `swarblock`,
    ///   - have always one swar per swarbeat
    fn build(&self, blks: &Vec<SwarBlock>) -> SwarBlocks {
        let mut octave_blks = Vec::<SwarBlock>::new();
        let first_blk = blks.first().unwrap();
        println!("first_blk: {}", first_blk);

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
        let blks = self.build(&self.aroha.0);
        self.lower = Some(blks.lower());
    }

    pub fn build_higher(&mut self) {
        let blks = self.build(&self.aroha.0);
        self.higher = Some(blks.higher());
    }

    pub fn lower(&self) -> Option<&SwarBlocks> {
        self.lower.as_ref()
    }

    pub fn higher(&self) -> Option<&SwarBlocks> {
        self.higher.as_ref()
    }

    pub fn aroha(&self) -> &SwarBlocks {
        &self.aroha
    }

    pub fn play(&self, dev: &AudioDevice) {
        self.aroha.play(&dev);
    }

    /// Returns aroha swars in all three octaves (in order):
    /// lower middle higher
    /// e.g.: For swars x y z, return .x .y .z x y z x. y. z.
    pub fn swars_in_all_octaves(&self) -> Vec<&Swar> {
        let mut swars = Vec::<&Swar>::new();
        for sw in self.lower.as_ref().unwrap().to_swars_as_ref() {
            swars.push(sw);
        }

        swars.extend(self.aroha.to_swars_as_ref());

        for sw in self.higher.as_ref().unwrap().to_swars_as_ref() {
            swars.push(sw);
        }

        swars
    }

    pub fn swars_by_context(&self, swar: &Swar) -> Option<Vec<&Swar>> {
        let all_swars = [
            self.lower.as_ref().unwrap(),
            &self.aroha,
            &self.higher.as_ref().unwrap()
        ];

        for blks in all_swars.iter() {
            if let Some(index) = blks.index_swar(&swar) {
                return blks.adjacent_swars(&index);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;

    /// test aroha as string
    #[test]
    fn test_aroha() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "S - R - M - P - D - S. - -";

        let aroha = raag.aroha();
        assert_eq!(aroha.aroha().to_string(), expected);
    }

    /// test aroha lower octave
    #[test]
    fn test_aroha_lower_octave() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = ".R - .M - .P - .D -";
        let aroha = raag.aroha().build(&raag.aroha().aroha.0);

        assert_eq!(aroha.lower().to_string(), expected);
    }

    /// test aroha higher octave
    #[test]
    fn test_aroha_higher_octave() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = "R. - M. - P. - D. -";
        let aroha = raag.aroha().build(&raag.aroha().aroha.0);

        assert_eq!(aroha.higher().to_string(), expected);
    }
}