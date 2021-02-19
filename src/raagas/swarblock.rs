use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::swar_beat::SwarBeat;
use std::fmt;
use crate::raagas::sound::{AudioDevice};
use rodio::{PlayError, Sink};
use crate::raagas::{Mutate, utils, MutationOperators};
use rand::seq::SliceRandom;
use crate::raagas::constants::{BPS, KAN_SWAR_BEAT_COUNT};
use rand::{Rng};

#[derive(Debug, Clone)]
pub struct SwarBlock(pub Vec<SwarBeat>);

// #[derive(Debug, Clone)]
// pub struct SwarBlockRef<'a>(pub Vec<&'a SwarBeat>);
//
#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

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

        return None;
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
        println!("=> playing swarblock: {}", self);
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
        for sw_bt in &self.0 {
            s = format!("{} {}", s, sw_bt);
        }

        write!(f, "{}", s)
    }
}

impl SwarBlocks {

    //return the first index of swar matched, this can be improved to pick a random
    //index for a list of indices
    pub fn index_swar(&self, swar: &Swar) -> Option<SwarInSwarBlock> {
        for blk in &self.0 {
            if let Some(index) =  blk.index_swar(swar) {
                return Some(index);
            }
        }

        None
    }

    pub fn swarbeats(&self) -> Vec<&SwarBeat> {
        let mut swar_beats = Vec::<&SwarBeat>::new();
        for blk in &self.0 {
            for sw_bt in &blk.0 {
                swar_beats.push(sw_bt);
            }
        }

        swar_beats
    }

    // return a random index and the associated swar
    pub fn random_swarbeat(&self) -> Option<(usize, &SwarBeat)> {
        let swar_beats = self.swarbeats();
        let mut rnd = rand::thread_rng();
        let i = rnd.gen_range(0, swar_beats.len());

        if let Some(sw_bt) = swar_beats.get(i as usize) {
            return Some((i, sw_bt));
        }

        None
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
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for blk in &self.0 {
            let mut blk_swars = blk.to_swars();
            swars.append(&mut blk_swars);
        }

        swars
    }

    pub fn adjacent_swars(&self, index: &SwarInSwarBlock) -> Option<Vec<&Swar>> {
        let mut swar_beats = Vec::<&SwarBeat>::new();
        for blk in &self.0 {
            for sw_bt in &blk.0 {
                swar_beats.push(sw_bt);
            }
        }
        let mut swars = Vec::<&Swar>::new();
        if let Some(sw_bt) = swar_beats.get(index.swarbeat_index) {
            if let Some(swar) = sw_bt.swars.get(index.swar_index) {
                // preceeding
                // if (sw_bt.swars.len() == -1) or first of this sw_bt, then
                // go to preceding sw_bt and get last swar
                // if (sw_bt.swars.len() == -1) or first of this sw_bt and
                // there are no preceding swars then None

                if (index.swar_index == 0) && (index.swarbeat_index >= 1) {
                    if let Some(prev_sw_bt) = swar_beats.get(index.swarbeat_index-1) {
                        // should we handle None case?
                        let pre_swar = prev_sw_bt.swars.last().unwrap();
                        swars.push(pre_swar);
                    }
                } else {
                    if index.swar_index > 0 {
                        let pre_swar = sw_bt.swars.get(index.swar_index-1).unwrap();
                        swars.push(pre_swar);
                    }
                }

                swars.push(swar);

                // succeeding
                if index.swar_index < sw_bt.swars.len() {
                    let post_swar = sw_bt.swars.get(index.swar_index+1).unwrap();
                    swars.push(post_swar);
                } else {
                    if index.swar_index+1 >= sw_bt.len() &&
                        index.swarbeat_index+1 < swar_beats.len() {
                        let next_sw_bt = swar_beats.get(index.swarbeat_index +1).unwrap();
                        let post_swar = next_sw_bt.swars.first().unwrap();
                        swars.push(post_swar);
                    }
                }
            }
            return Some(swars);
        }

        None
    }

    pub fn is_monotonic_increasing(&self, match_swars: &Vec<&Swar>) -> bool {
        let mut swars = Vec::<&Swar>::new();
        for blk in &self.0 {
            for sw_bt in &blk.0 {
                 for sw in &sw_bt.swars {
                     swars.push(sw);
                 }
            }
        }
        let get_swar_index = |swars: &Vec<&Swar>, swar: &Swar| -> Option<usize> {
            for (i, sw) in swars.iter().enumerate() {
                let swar_freq = swar.freq();
                if sw.freq() == swar_freq {
                    return Some(i);
                }
            }
            None
        };

        let first_swar = match_swars.first().unwrap();
        if let Some(first_sw_i) = get_swar_index(&swars, first_swar) {
            // there should be tail
            let match_tail = match_swars.get(1..).unwrap();
            for sw in match_tail {
                if let Some(next_sw_i) = get_swar_index(&swars, sw) {
                    if next_sw_i < first_sw_i {
                        return false;
                    }
                }
            }

            return true;
        }

        false

    }

    // pub fn to_swarblock(&self) -> SwarBlockRef {
    //     let swar_beats = self.swarbeats();
    //     SwarBlockRef(swar_beats)
    // }

    pub fn play(&self, dev: &AudioDevice) {
        for blk in &self.0 {
            blk.play(&dev);
        }
    }

    pub fn get_swarbeat_mut_at(&mut self, index: usize) -> Option<&mut SwarBeat> {
        let mut i: usize = 0;
        for blk in &mut self.0 {
            for sw_bt in &mut blk.0 {
                if i == index {
                    return Some(sw_bt);
                }
                i += 1;
            }
        }

        None
    }

    pub fn replace_swar(&mut self, index: &SwarInSwarBlock, swar: Swar) {
        if let Some(sw_bt) = self.get_swarbeat_mut_at(index.swarbeat_index) {
            sw_bt.replace_swar(index.swar_index, swar);
        }
    }

    pub fn insert_swar(&mut self, index: &SwarInSwarBlock, swar: Swar) {
        if let Some(sw_bt) = self.get_swarbeat_mut_at(index.swarbeat_index) {
            if sw_bt.len() < 4 {
                sw_bt.insert_swar(index.swar_index+1, swar);
            } else {
                // try the swarbeat before or after
            }
        }
    }
}

impl Mutate for SwarBlocks {
    fn mutate(&self,
              index: &SwarInSwarBlock,
              from: Vec<&Swar>) -> SwarBlocks {

        self.mutate_swar(index, from)
    }

    fn mutate_swar(&self, index: &SwarInSwarBlock, from: Vec<&Swar>) -> SwarBlocks {
        let mut rnd = rand::thread_rng();
        let rnd_swar_from = from.choose(&mut rnd);
        let rnd_swar = rnd_swar_from.unwrap().to_owned();
        let swar_mut_type = rnd_swar.random_mutation_operator();

        let mut mut_sw_blk = self.clone();
        // let mut swar_beats = mut_sw_blk.swarbeats();
        // mutations for Y in X Y Z -- X, Y, Z, X:Y, Y:X, Y:Z, Z:Y, X:X, Y:Y, Z:Z

        match swar_mut_type.as_str() {
            "simple" => {
                mut_sw_blk.replace_swar(&index, rnd_swar.clone());
            },
            "inc_beat" => {
                // mut_sw_blk.replace_swar_with_beat_change(&index, &rnd_swar, 1.0);
                if let Some(sw_bt) = mut_sw_blk.get_swarbeat_mut_at(index.swarbeat_index) {
                    let mut rnd_swar = rnd_swar.clone();
                    rnd_swar.inc_beat_count(1.0);
                    sw_bt.replace_swar(index.swar_index, rnd_swar);
                }
            },
            "dec_beat" => {
                let mut dec_bt = rnd_swar.beat_cnt;
                if dec_bt >= 1.0 {
                    dec_bt = dec_bt/2.0;
                }
                if let Some(sw_bt) = mut_sw_blk.get_swarbeat_mut_at(index.swarbeat_index) {
                    let mut rnd_swar = rnd_swar.clone();
                    rnd_swar.dec_beat_count(dec_bt);
                    sw_bt.replace_swar(index.swar_index, rnd_swar);
                }
            },
            "share_beat" => {
                // two swars share a beat
                // X Y Z -> X Y:_ Z
                // TODO: ensure the beat is a full beat, how to handle beat: S:G:M:P

                let mut rnd_swar_a = rnd_swar.clone();
                rnd_swar_a.set_beat_count(0.5);
                let mut rnd_swar_b = from.choose(&mut rnd).unwrap().to_owned().clone();
                rnd_swar_b.set_beat_count(0.5);

                // now randomly insert/replace rnd_swar and rnd_swar_latter
                let b = rand::thread_rng().gen_bool(0.5);
                if b {
                    mut_sw_blk.replace_swar(&index, rnd_swar_a);
                    mut_sw_blk.insert_swar(&index, rnd_swar_b);
                } else {
                    mut_sw_blk.insert_swar(&index, rnd_swar_b);
                    mut_sw_blk.replace_swar(&index, rnd_swar_a);
                }
            },
            "kan_swar" => {
                // two swars share a beat, and one swar being a kan swar
                // X Y Z -> X _/_ Z
                // TODO: ensure the beat is a full beat, how to handle beat: S:G:M:P

                let mut rnd_swar_a = rnd_swar.clone();
                rnd_swar_a.set_beat_count(KAN_SWAR_BEAT_COUNT);
                let mut rnd_swar_b = from.choose(&mut rnd).unwrap().to_owned().clone();
                rnd_swar_b.set_beat_count(1.0 - KAN_SWAR_BEAT_COUNT);

                // now randomly insert/replace rnd_swar and rnd_swar_latter
                let b = rand::thread_rng().gen_bool(0.5);
                if b {
                    mut_sw_blk.replace_swar(&index, rnd_swar_a);
                    mut_sw_blk.insert_swar(&index, rnd_swar_b);
                } else {
                    mut_sw_blk.insert_swar(&index, rnd_swar_b);
                    mut_sw_blk.replace_swar(&index, rnd_swar_a);
                }
            },
            _ => {}
        }
        println!("[{}] mutated: {:?}", swar_mut_type, mut_sw_blk);

        mut_sw_blk
    }

    fn mutate_swar_duration(&self, _i: usize) -> Option<Swar> {
        None
    }
}
