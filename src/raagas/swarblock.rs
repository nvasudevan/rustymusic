use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::swarbeat::SwarBeat;
use std::fmt;
use crate::raagas::sound::{AudioDevice};
use rodio::{PlayError, Sink};
use crate::raagas::utils;
use crate::raagas::constants::BPS;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct SwarBlock(pub Vec<SwarBeat>);

#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

#[derive(Debug)]
pub struct SwarInSwarBlock<'a> {
    pub swarbeat_index: usize,
    pub swar_index: usize,
    pub swar: &'a Swar
}

impl SwarBlock {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // retrieve the first (i, j) of match_swar, where
    // i is the matched swarbeat, and j is the index of swar within it
    pub fn index_swar(&self, match_swar: &Swar) -> Option<SwarInSwarBlock> {
        let swar_beats = &self.0;
        for (i, sw_bt) in swar_beats.into_iter().enumerate() {
            let sw_bt_swars = &sw_bt.swars;
            for (j, swar) in sw_bt_swars.into_iter().enumerate() {
                if swar.eq(&match_swar) {
                    return Some(SwarInSwarBlock {
                        swarbeat_index: i,
                        swar_index: j,
                        swar
                    });
                }
            }
        }
        None
    }

    // this is useful when we need when checking if the swars are in
    // ascending or descending order irrespective of the beat duration
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for sw_bt in &self.0 {
            for sw in &sw_bt.swars {
                swars.push(sw.clone());
            }
        }
        swars
    }

    pub fn random_swar_index(&self) -> Option<SwarInSwarBlock> {
        if let Some((i, sw_bt)) = self.random_swarbeat() {
            let swars = &sw_bt.swars;
            if swars.len() == 0 {
                let res = SwarInSwarBlock {
                    swarbeat_index: i,
                    swar_index: 0,
                    swar: swars.get(0).unwrap()
                };

                return Some(res);
            }

            let mut rnd = rand::thread_rng();
            let j_swar = rnd.gen_range(0, swars.len());

            let res = SwarInSwarBlock {
                swarbeat_index: i,
                swar_index: j_swar,
                swar: swars.get(j_swar).unwrap()
            };

            return Some(res);
        }

        None
    }

    // return a random index and the associated swar
    pub fn random_swarbeat(&self) -> Option<(usize, &SwarBeat)> {
        let mut rnd = rand::thread_rng();
        let i = rnd.gen_range(0, self.len());

        if let Some(sw_bt) = self.0.get(i as usize) {
            return Some((i, sw_bt));
        }

        None
    }

    pub fn build_sink(&self,
                      beat_src: &Option<BeatSrc>,
                      dev: &AudioDevice) -> Result<Vec<Option<Sink>>, PlayError> {
        let mut sinks: Vec<Option<Sink>> = Vec::new();
        for sw_bt in &self.0 {
            for sw in &sw_bt.swars {
                let bt_sink = sw.build_sink(&beat_src, &dev)?;
                sinks.push(bt_sink);
            }
        }

        Ok(sinks)
    }

    pub fn play(&self, dev: &AudioDevice) {
        for sw_bt in &self.0 {
            for sw in &sw_bt.swars {
                let sw_sink = sw.build_sink(&None, &dev).unwrap();
                match sw_sink {
                    Some(sink) => {
                        sink.play();
                        utils::delay(sw.beat_cnt * BPS);
                        sink.stop();
                    },
                    None => {
                        utils::delay(sw.beat_cnt * BPS);
                    }
                }
            }
        }

    }
}
impl fmt::Display for SwarBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if let Some(first_sw_bt) = &self.0.first() {
            s = format!("{}{}", s, first_sw_bt);
            if let Some(rest_swars) = self.0.get(1..) {
                for sw_bt in rest_swars {
                    s = format!("{} {}", s, sw_bt);
                }
            }
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::swarblock::SwarBlock;
    use crate::raagas::raag::load;

    /// test rendering of a swar spanning two swarbeats, e.g.: M:M -:P
    #[test]
    fn test_load_swar_spanning_two_swarbeats() {
        let s = "M:M -:P";
        let swarbeats = load::to_swarbeats(s);
        let blk: SwarBlock = SwarBlock(swarbeats);
        assert_eq!(blk.to_string(), "M:M -:P");
    }

    /// test rendering of sequence of swarbeats with different combination of swars and beats
    /// S:R:M:P S - - M P:P -:D :D P/M
    #[test]
    fn test_load_multiple_swarbeats() {
        let s = "S:R:M:P S - - M P:P -:D :D P/M";
        let swarbeats = load::to_swarbeats(s);
        let blk: SwarBlock = SwarBlock(swarbeats);
        assert_eq!(blk.to_string(), "S:R:M:P S - - M P:P -:D :D P/M");
    }
}