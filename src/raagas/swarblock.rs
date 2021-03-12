use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::swarbeat::SwarBeat;
use std::fmt;
use crate::raagas::sound::{AudioDevice, Pitch};
use rodio::{PlayError, Sink};
use crate::raagas::utils;
use crate::raagas::constants::{BPS, KAN_SWAR_BEAT_COUNT};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct SwarBlock(pub Vec<SwarBeat>);

#[derive(Debug)]
pub struct SwarInSwarBlock<'a> {
    pub swarbeat_index: usize,
    pub swar_index: usize,
    pub swar: &'a Swar
}

impl SwarBlock {
    /// Returns the number of swarbeats in the swarblock
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn lower(&self) -> SwarBlock {
        let mut lower = Vec::<SwarBeat>::new();
        for bt in &self.0 {
            let lower_bt = bt.lower();
            lower.push(lower_bt);
        }

        SwarBlock(lower)
    }

    /// Retrieve the first (i, j) of match_swar, where
    /// i is the matched swarbeat, and j is the index of swar within it
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

    pub fn to_swars_as_ref(&self) -> Vec<&Swar> {
        let mut swars = Vec::<&Swar>::new();
        for sw_bt in &self.0 {
            for sw in &sw_bt.swars {
                swars.push(sw);
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

impl From<Vec<Swar>> for SwarBlock {
    /// Return a swarblock from a sequence of swars.
    fn from(swars: Vec<Swar>) -> Self {
        let mut swar_bts = Vec::<SwarBeat>::new();
        swar_bts.push(SwarBeat::new(swars));

        SwarBlock(swar_bts)
    }
}

impl From<&str> for SwarBlock {
    /// Return a SwarBlock from a string denoting sequence of swars
    fn from(s: &str) -> Self {
            let mut swarbeats_vec: Vec<SwarBeat> = vec![];
            let swarbeats: Vec<String> = s.trim().split(" ").map(|x| x.to_string()).collect();
            for sw_bt in swarbeats {
                let mut swars = Vec::<Swar>::new();
                if sw_bt.eq("-") {
                    // S:G -  (G will be a beat and a half)
                    // S - -  Go all the way back to S and extend it by 1 beat
                    // add an extra beat to the previous swarbeat
                    // and nothing to add for current swarbeat
                    extend_last_swar(&mut swarbeats_vec, 1.0);
                } else {
                    if sw_bt.contains(":") {
                        // four cases: either two swars or four swars
                        // two swars: S:S, -:S :S S: we don't need 'S:- M' as we can always write it as
                        // 'S:M -'
                        // four swars: S:S:S:S
                        let sw_bts_vec: Vec<String> = sw_bt.split(":").map(|x| x.to_string()).collect();
                        if sw_bts_vec.len() == 2 {
                            // first and second swar are each 0.5 beat
                            let first_swar_s = sw_bts_vec.first().unwrap();
                            let last_swar_s = sw_bts_vec.last().unwrap();
                            if first_swar_s.eq("-") {
                                // modify last swar of previous SwarBeat
                                extend_last_swar(&mut swarbeats_vec, 0.5);
                                // and last_swar to swars
                                let last_swar = Swar::new(Pitch::new(last_swar_s.to_string()), 0.5);
                                swars.push(last_swar);
                            } else if last_swar_s.eq("-") {
                                // add the first swar
                                let first_swar = Swar::new(Pitch::new(last_swar_s.to_string()), 1.0);
                                swars.push(first_swar);
                            } else {
                                if first_swar_s.eq("") {
                                    swars.push(Swar::empty(0.5));
                                } else {
                                    swars.push(Swar::new(
                                        Pitch::new(first_swar_s.to_string()),
                                        0.5)
                                    );
                                }
                                if last_swar_s.eq("") {
                                    swars.push(Swar::empty(0.5));
                                } else {
                                    swars.push(Swar::new(
                                        Pitch::new(last_swar_s.to_string()),
                                        0.5)
                                    );
                                }
                            }
                        } else if sw_bts_vec.len() == 4 {
                            // each of the four swars are 0.25 beat
                            let beat_count = 0.25;
                            for sw in sw_bts_vec {
                                if sw.eq("-")  {
                                    extend_last_swar(&mut swarbeats_vec, beat_count);
                                } else {
                                    let swar = Swar::new(
                                        Pitch::new(sw.to_string()), beat_count
                                    );
                                    swars.push(swar);
                                }
                            }
                        }
                    } else if sw_bt.contains("/") {
                        // kan swar
                        // e.g.: P/M
                        let swrs: Vec<String> = sw_bt.split("/").map(|x| x.to_string()).collect();
                        let kan = swrs.get(0).unwrap();
                        let kan_bt_cnt: f32 = 1.0 * KAN_SWAR_BEAT_COUNT;
                        swars.push(Swar::new(Pitch::new(kan.to_string()), kan_bt_cnt));
                        let main_swar = swrs.get(1).unwrap();
                        swars.push(Swar::new(Pitch::new(main_swar.to_string()), 1.0 - kan_bt_cnt));

                    } else {
                        // all else, just a plain swar (e.g.: S)
                        swars.push(Swar::new(Pitch::new(sw_bt.to_string()), 1.0));
                    }
                }
                swarbeats_vec.push(SwarBeat::new(swars));
            }

            SwarBlock(swarbeats_vec)
    }
}

// traversing from the last item, returns the index of the swarbeat with a swar in it
// S - - M - -, will return 3 (as the last swarbeat is a '-' and M is the one with swar
// from the back
fn get_last_swarbeat_with_swar(swarbeats: &mut Vec<SwarBeat>) -> Option<usize> {
    // (0..n) is excluded and we only need 0 to n-1
    for i in (0..swarbeats.len()).rev() {
        let sw_bt = swarbeats.get(i).unwrap();
        if sw_bt.len() > 0 {
            return Some(i);
        }
    }

    None
}

// increment the last swar of previous swarbeat
fn extend_last_swar(swarbeats: &mut Vec<SwarBeat>, beat_count_inc: f32) {
    if let Some(i) = get_last_swarbeat_with_swar(swarbeats) {
        if let Some(prev_sw_bt) = swarbeats.get_mut(i) {
            prev_sw_bt.increment_swar_at(prev_sw_bt.len()-1, beat_count_inc);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::swarblock::{SwarBlock, extend_last_swar, get_last_swarbeat_with_swar};

    /// test no of swarbeats match for a sequence
    #[test]
    fn test_string_to_swarbeats_count() {
        let s = "S:R:M:P S - - M P:P -:D :D P/M";
        let blk = SwarBlock::from(s);
        assert_eq!(blk.len(), 9);
    }

    /// test if the beat count matches when a swar is extended
    #[test]
    fn test_extend_swar_with_beat() {
        let s = "S - G -";
        let blk = SwarBlock::from(s);
        let mut swarbeats = blk.0;
        extend_last_swar(&mut swarbeats, 1.0);
        let i = get_last_swarbeat_with_swar(&mut swarbeats).unwrap();
        let sw_bt = swarbeats.get(i).unwrap();
        let last_swar = sw_bt.swars.last().unwrap();
        assert_eq!(last_swar.beat_cnt, 3.0);
    }

    /// test if the beat count matches when half a beat swar is extended
    #[test]
    fn test_extend_swar_with_half_beat() {
        let s = "S - :G";
        let blk = SwarBlock::from(s);
        let mut swarbeats = blk.0;
        extend_last_swar(&mut swarbeats, 1.0);
        let i = get_last_swarbeat_with_swar(&mut swarbeats).unwrap();
        let sw_bt = swarbeats.get(i).unwrap();
        let last_swar = sw_bt.swars.last().unwrap();
        assert_eq!(last_swar.beat_cnt, 1.5);
    }

    /// test rendering of a swar spanning two swarbeats, e.g.: M:M -:P
    #[test]
    fn test_load_swar_spanning_two_swarbeats() {
        let s = "M:M -:P";
        let blk: SwarBlock = SwarBlock::from(s);
        assert_eq!(blk.to_string(), "M:M -:P");
    }

    /// test rendering of sequence of swarbeats with different combination of swars and beats
    /// S:R:M:P S - - M P:P -:D :D P/M
    #[test]
    fn test_load_multiple_swarbeats() {
        let s = "S:R:M:P S - - M P:P -:D :D P/M";
        let blk: SwarBlock = SwarBlock::from(s);
        assert_eq!(blk.to_string(), "S:R:M:P S - - M P:P -:D :D P/M");
    }
}