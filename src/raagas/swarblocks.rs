use crate::raagas::swarblock::{SwarInSwarBlock, SwarBlock};
use crate::raagas::swarbeat::SwarBeat;
use crate::raagas::swars::Swar;
use rand::Rng;
use crate::raagas::constants::KAN_SWAR_BEAT_COUNT;
use crate::raagas::{Mutate, MutationOperators};
use crate::raagas::sound::AudioDevice;
use std::fmt;
use rand::seq::SliceRandom;


#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

impl SwarBlocks {

    /// return the lower octave equivalent of the swars
    pub fn lower(&self) -> SwarBlocks {
        let mut lower_blks = Vec::<SwarBlock>::new();
        for blk in &self.0 {
            let lower_blk = blk.lower();
            lower_blks.push(lower_blk);
        }

        SwarBlocks(lower_blks)
    }

    /// Retrieve the first index of swar matched.
    /// This can be improved to pick a random index for a list of indices
    pub fn index_swar(&self, swar: &Swar) -> Option<SwarInSwarBlock> {
        for blk in &self.0 {
            if let Some(index) =  blk.index_swar(swar) {
                return Some(index);
            }
        }

        None
    }

    /// Returns a list of swarbeats from the swarblocks.
    pub fn swarbeats(&self) -> Vec<&SwarBeat> {
        let mut swar_beats = Vec::<&SwarBeat>::new();
        for blk in &self.0 {
            for sw_bt in &blk.0 {
                swar_beats.push(sw_bt);
            }
        }

        swar_beats
    }

    /// Traversing in reverse from `from`, returns the index of the swarbeat with a swar in it
    ///
    /// - for S - M -, from `3` will return (2,0,M)
    /// - for S - M:- -, from `3`, this is never possible
    /// - for S - M: -, from `3` will return (2,0,M)
    pub fn swar_index_reverse(&self, from_sw_bt: usize, from_sw: usize) -> Option<SwarInSwarBlock> {
        let swarbeats = self.swarbeats();

        // first deal with current swarbeat and then traverse backwards
        let curr_sw_bt = swarbeats.get(from_sw_bt).unwrap();
        if let Some((j, swar)) = curr_sw_bt.swar_index_reverse_from(from_sw) {
            return Some(SwarInSwarBlock {
                swarbeat_index: from_sw_bt,
                swar_index: j,
                swar: &swar
            });
        }

        // [0..from_sw_bt) -- from_sw_bt is excluded
        for i in (0..from_sw_bt).rev() {
            let sw_bt = swarbeats.get(i).unwrap();
            println!("[{}] - {}", i, sw_bt);
            if let Some((j, swar)) = sw_bt.swar_index_reverse() {
                return Some(SwarInSwarBlock {
                    swarbeat_index: i,
                    swar_index: j,
                    swar: &swar
                });
            }
        }

        None
    }

    fn swar_index_forward(&self, from_sw_bt: usize, from_sw: usize) -> Option<SwarInSwarBlock> {
        let swarbeats = self.swarbeats();

        // first deal with current swarbeat and then traverse forwards
        let curr_sw_bt = swarbeats.get(from_sw_bt).unwrap();
        if let Some((j, swar)) = curr_sw_bt.swar_index_forward_from(from_sw) {
            return Some(SwarInSwarBlock {
                swarbeat_index: from_sw_bt,
                swar_index: j,
                swar: &swar
            });
        }

        // [i..j) -- j is excluded
        for i in from_sw_bt+1..swarbeats.len() {
            let sw_bt = swarbeats.get(i).unwrap();
            println!("[{}] - {}", i, sw_bt);
            if let Some((j, swar)) = sw_bt.swar_index_forward() {
                return Some(SwarInSwarBlock {
                    swarbeat_index: i,
                    swar_index: j,
                    swar: &swar
                });
            }
        }

        None
    }

    /// Return a random swarbeat and its index. Swarbeats with empty swars are excluded.
    pub fn random_swarbeat(&self) -> Option<(usize, &SwarBeat)> {
        let swar_beats = &self.swarbeats();
        let mut indices = Vec::<usize>::new();
        for (i, sw_bt) in swar_beats.into_iter().enumerate() {
            if sw_bt.len() > 0 {
                indices.push(i);
            }
        }
        println!("indices: {:?}", indices);

        if let Some(i) = indices.choose(&mut rand::thread_rng()) {
            return Some((*i, &swar_beats.get(*i).as_ref().unwrap()));
        }

        None
    }

    pub fn random_swar_index(&self) -> Option<SwarInSwarBlock> {
        if let Some((i, sw_bt)) = self.random_swarbeat() {
            // let i = 13;
            println!("rnd sw_bt: {}", i);
            let swars = &sw_bt.swars;
            println!("swars: {:?}", swars);
            // TODO: handle cases when index falls on a '-'

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

    /// Returns a list of swars for the swarblocks.
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for blk in &self.0 {
            let mut blk_swars = blk.to_swars();
            swars.append(&mut blk_swars);
        }

        swars
    }

    pub fn to_swars_as_ref(&self) -> Vec<&Swar> {
        let mut swars = Vec::<&Swar>::new();
        for blk in &self.0 {
            let mut blk_swars = blk.to_swars_as_ref();
            swars.append(&mut blk_swars);
        }

        swars
    }

    /// Returns the adjacent swars surrounding the index.
    pub fn adjacent_swars(&self, index: &SwarInSwarBlock) -> Option<Vec<&Swar>> {
        println!("index: {:?}", index);
        let swar_beats = self.swarbeats();
        let mut swars = Vec::<&Swar>::new();
        if let Some(sw_bt) = swar_beats.get(index.swarbeat_index) {
            if let Some(swar) = sw_bt.swars.get(index.swar_index) {
                // preceeding
                // if (sw_bt.swars.len() == -1) or first of this sw_bt, then
                // go to preceding sw_bt and get last swar
                // if (sw_bt.swars.len() == -1) or first of this sw_bt and
                // there are no preceding swars then None

                if let Some(index) = self.swar_index_reverse(
                    index.swarbeat_index,
                    index.swar_index-1
                ) {
                    let prev_sw_bt = swar_beats.get(index.swarbeat_index).unwrap();
                    let pre_swar = prev_sw_bt.swars.get(index.swar_index).unwrap();
                    swars.push(pre_swar);
                }

                swars.push(swar);

                // succeeding
                if let Some(index) = self.swar_index_forward(
                    index.swarbeat_index,
                    index.swar_index
                ) {
                    let next_sw_bt = swar_beats.get(index.swarbeat_index).unwrap();
                    let next_swar = next_sw_bt.swars.get(index.swar_index).unwrap();
                    swars.push(next_swar);
                }
            }

            return Some(swars);
        }

        None
    }

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

impl From<Vec<Swar>> for SwarBlocks {
    /// Returns a Swarblocks for a given sequence of swars
    fn from(swars: Vec<Swar>) -> SwarBlocks {
        let mut blks = Vec::<SwarBlock>::new();
        blks.push(SwarBlock::from(swars));

        SwarBlocks(blks)
    }
}

impl fmt::Display for SwarBlocks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        if let Some(first_blk) = self.0.first() {
            s = format!("{}{}", s, first_blk);
            if let Some(rest_blks) = self.0.get(1..) {
                for blk in rest_blks {
                    s = format!("{} {}", s, blk);
                }
            }
        }

        write!(f, "{}", s)
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

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;

    #[test]
    fn test_prev_swar_in_swarblocks() {
        let raag = "durga";
        let composition = "durga";
        let raag = load::load_yaml(raag, composition).unwrap();
        let blks = raag.pakad().as_ref().unwrap();
        println!("pakad: {}", blks);
        let from_sw_bt = 15;
        let from_sw = 0;
        let prev_swar = blks.swar_index_reverse(from_sw_bt, from_sw);
        assert!(prev_swar.is_some());
        assert_eq!(prev_swar.unwrap().swarbeat_index, 13);
    }

    #[test]
    fn test_prev_swar_from_blank_swar_in_swarblocks() {
        let raag = "malkauns";
        let composition = "comp1";
        let line = "lineB";

        let raag = load::load_yaml(raag, composition).unwrap();
        let blks = raag.swarmaalika().sthayi.lines.get(line).unwrap();
        println!("blks: {}", blks);

        let from_sw_bt = 12;
        let from_sw = 0;
        let prev_swar = blks.swar_index_reverse(from_sw_bt, from_sw);
        println!("prev swar: {:?}", prev_swar);
        assert!(prev_swar.is_some());
        assert_eq!(prev_swar.unwrap().swarbeat_index, 11);
    }

}
