use crate::raagas::swars::Swar;
use crate::raagas::swarblocks::SwarBlocks;
use crate::raagas::sound::AudioDevice;
use crate::raagas::swarblock::SwarBlock;
use crate::raagas::swarbeat::SwarBeat;

/// Aroha in all three octaves
#[derive(Clone)]
pub struct Aroha {
   aroha: SwarBlocks,
   lower: SwarBlocks,
   higher: SwarBlocks
}

impl Aroha {
    pub fn new(aroha: SwarBlocks) -> Self {
        let aroha_swars = aroha.to_swars_as_ref();
        let lower = Vec::<SwarBlocks>::new();

        // take out the first and last swar
        let first_blk = aroha.0.first().unwrap();
        let first_beat = first_blk.0.first().unwrap();
        let mut lower_sw_blks = Vec::<SwarBlock>::new();

        // finish up the swar in the first beat
        let mut lower_sw_beats = Vec::<SwarBeat>::new();
        if let Some(first_beat_swars) = first_beat.swars.get(1..) {
            let lower_first_beat_swars = first_beat_swars.to_vec();
            let lower_first_beat = SwarBeat::new(lower_first_beat_swars);
            lower_sw_beats.push(lower_first_beat);
        }
        // add the remaining swarbeats from first_blk
        if aroha.0.len() > 1 {
            //multiple swarblocks e.g. raag malkauns

            // add the remaining beats of first block
            if let Some(first_blk_remaining_beats) = first_blk.0.get(1..) {
                let mut first_blk_remaining_beats_vec = first_blk_remaining_beats.to_vec();
                lower_sw_beats.append(&mut first_blk_remaining_beats_vec);
            }
            lower_sw_blks.push(SwarBlock(lower_sw_beats));

            let middle_aroha_blks = aroha.0.get(1..).unwrap();
            let mut middle_aroha_blks_vec = middle_aroha_blks.to_vec();

            // get the last swarblock, and lose its last swar
            let last_blk = middle_aroha_blks_vec.pop().unwrap();

            // add all the blks others than the last
            if ! middle_aroha_blks_vec.is_empty() {
                lower_sw_blks.append(&mut middle_aroha_blks_vec);
            }

            // process last
            let mut last_swar_beats = last_blk.0;
            let mut last_beat = last_swar_beats.pop().unwrap();
            let last_beat_swars = last_beat.swars.pop();

            // now add this last blk
            let last_blk = SwarBlock(last_swar_beats);
            lower_sw_blks.push(last_blk);

        } else {
            // single swarblock
            // lose the last swar of last swarbeat

        }


        let mut higher = Vec::<Swar>::new();
        for sw in aroha_swars.get(1..(aroha_swars.len()-1)).unwrap() {
            let higher_sw = sw.higher();
            higher.push(higher_sw);
        }

        Aroha {
           aroha,
            lower,
            higher: SwarBlocks::from(higher),
        }
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
        for sw in self.lower.to_swars_as_ref() {
            swars.push(sw);
        }

        swars.extend(self.aroha.to_swars_as_ref());

        for sw in self.higher.to_swars_as_ref() {
            swars.push(sw);
        }

        swars
    }

    pub fn all_octaves(&self) -> Vec<&SwarBlocks> {
        let mut all_swarblocks = Vec::<&SwarBlocks>::new();
        all_swarblocks.push(&self.lower);
        all_swarblocks.push(&self.aroha);
        all_swarblocks.push(&self.higher);

        all_swarblocks
    }

}

/// Avroha in all three octaves
#[derive(Clone)]
pub struct Avroha {
    avroha: SwarBlocks,
    lower: SwarBlocks,
    higher: SwarBlocks
}

impl Avroha {
    /// Returns an Avroha along with the lower and the higher
    /// octave equivalent
    pub fn new(avroha: SwarBlocks) -> Self {
        let avroha_swars = avroha.to_swars_as_ref();
        let lower = avroha.lower();

        let mut higher = Vec::<Swar>::new();
        for sw in avroha_swars.get(1..(avroha_swars.len()-1)).unwrap() {
            let higher_sw = sw.higher();
            higher.push(higher_sw);
        }

        Avroha {
            avroha,
            lower,
            higher: SwarBlocks::from(higher),
        }
    }

    pub fn avroha(&self) -> &SwarBlocks {
        &self.avroha
    }

    /// Returns avroha swars in all three octaves (in order):
    /// higher middle lower
    /// e.g.: For swars x y z, return x. y. z. x y z .x .y .z
    pub fn swars_in_all_octaves(&self) -> Vec<&Swar> {
        let mut swars = Vec::<&Swar>::new();
        for sw in self.higher.to_swars_as_ref() {
            swars.push(sw);
        }

        swars.extend(self.avroha.to_swars_as_ref());

        for sw in self.lower.to_swars_as_ref() {
            swars.push(sw);
        }

        swars
    }

    pub fn all_octaves(&self) -> Vec<&SwarBlocks> {
        let mut all_swarblocks = Vec::<&SwarBlocks>::new();
        all_swarblocks.push(&self.higher);
        all_swarblocks.push(&self.avroha);
        all_swarblocks.push(&self.lower);

        all_swarblocks

        // for blk in self.higher.0 {
        //     swar_blocks.push(blk);
        // }
        // for blk in self.avroha.0 {
        //     swar_blocks.push(blk);
        // }
        // for blk in self.lower.0 {
        //     swar_blocks.push(blk);
        // }

    }

    pub fn play(&self, dev: &AudioDevice) {
        self.avroha.play(&dev);
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

    /// test aroha all octaves
    #[test]
    fn test_aroha_all_octaves() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let expected = ".R - .M - .P - .D -";

        let aroha = raag.aroha();
        let aroha_lower = &aroha.lower;
        let aroha_higher = &aroha.higher;

        assert_eq!(aroha_lower.to_string(), expected);
    }
}