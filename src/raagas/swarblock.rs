use crate::raagas::swars::{Swar, BeatSrc};
use std::fmt;
use std::iter::FromIterator;
use crate::raagas::sound::{AudioDevice, TimedSink};
use rodio::PlayError;
use crate::raagas::{Mutate, utils, SwarBlockMutationType, MutationOperators};
use rand::seq::SliceRandom;
use crate::raagas::constants::{BPS, KAN_SWAR_BEAT_COUNT};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

impl SwarBlock {
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for sw in &self.0 {
            swars.push(sw.clone());
        }
        swars
    }

    pub fn no_beats(&self) -> usize {
        let mut cnt: usize = 0;
        for sw in &self.0 {
            cnt = cnt + sw.beat_cnt as usize;
        }

        cnt
    }

    pub fn adjacent_swars(&self, index: usize) -> Option<Vec<Swar>> {
        let swars = &self.0;
        let no_swars = swars.len();

        if index > no_swars-1 {
            return None;
        }

        let build_swars = |p: usize, q: usize| {
            (p..q+1).map(|i| swars.get(i).unwrap().clone()).collect()
        };

        if index == 0 {
            let _swars = swars.get(0..3).unwrap();
            return Some(_swars.to_vec());
        }

        // return the last swar and the penultimate
        if index == no_swars-1 {
            return Some(build_swars(index-2, index));
        }

        Some(build_swars(index-1, index+1))
    }

    pub fn random_swar(&self) -> Swar {
        let mut rnd = rand::thread_rng();
        let from = self.to_swars();
        let rnd_swar = from.choose(&mut rnd);

        let swar = rnd_swar.unwrap();

        swar.clone()
    }

    pub fn build_sink(&self,
                      beat_src: &Option<BeatSrc>,
                      dev: &AudioDevice,
                      vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        let mut sinks: Vec<TimedSink> = Vec::new();
        for bt in &self.0 {
            let bt_sink = bt.build_sink(&beat_src, &dev, vol)?;
            sinks.push(bt_sink);
        }

        Ok(sinks)
    }

    pub fn play(&self, dev: &AudioDevice, vol: f32) {
        for sw in &self.0 {
            let tsink = sw.build_sink(&None, &dev, vol).unwrap();
            tsink.sink.play();
            utils::delay(tsink.duration * BPS);
            tsink.sink.stop();
        }

    }
}
impl fmt::Display for SwarBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let first_swar = self.0.get(0).unwrap();
        s = format!("{} {}", s, first_swar);
        let mut prev_bt_cnt: f32 = first_swar.beat_cnt;
        let rest_swars = self.0.get(1..).unwrap();
        for swar in rest_swars {
            match swar.beat_cnt {
                0.5 => {
                    if prev_bt_cnt == 0.5 {
                        s = format!("{}{}", s, swar);
                    } else {
                        s = format!("{} {}:", s, swar);
                    }
                },
                KAN_SWAR_BEAT_COUNT => {
                    s = format!("{} {}/", s, swar);
                },
                _ => {
                    if prev_bt_cnt == KAN_SWAR_BEAT_COUNT {
                        s = format!("{}{}", s, swar);
                    } else if prev_bt_cnt == 0.5 {
                        s = format!("{}{}", s, swar);
                    } else {
                        s = format!("{} {}", s, swar);
                    }
                }
            }
            prev_bt_cnt = swar.beat_cnt;
        }

        write!(f, "{}", s)
    }
}

impl FromIterator<usize> for SwarBlock {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        let mut _blk: Vec<Swar> = Vec::new();
        for _ in iter {
            _blk.push(Swar {
                pitch: None,
                beat_cnt: 1.0,
            });
        }

        SwarBlock(_blk)
    }
}

impl SwarBlocks {
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for blk in &self.0 {
            let mut blk_swars = blk.to_swars();
            swars.append(&mut blk_swars);
        }

        swars
    }

    pub fn adjacent_swars(&self, index: usize) -> Option<Vec<Swar>> {
        let swarblk = &self.to_swarblock();
        swarblk.adjacent_swars(index)
    }

    pub fn to_swarblock(&self) -> SwarBlock {
        SwarBlock(self.to_swars())
    }

    pub fn random_swar(&self) -> Swar {
        self.to_swarblock().random_swar()
    }

    pub fn play_rt(&self, dev: &AudioDevice, vol: f32) {
        self.to_swarblock().play(&dev, vol)
    }
}

impl Mutate for SwarBlocks {
    fn mutate(&self, i: usize, mut_type: SwarBlockMutationType, from: Option<Vec<Swar>>) -> Option<SwarBlock> {
        self.to_swarblock().mutate(i, mut_type, from)
    }

    fn mutate_swar(&self, _i: usize, _from: Option<Vec<Swar>>) -> Option<SwarBlock> {
        None
    }

    fn mutate_swar_duration(&self, _i: usize) -> Option<Swar> {
        None
    }
}

impl Mutate for SwarBlock {
    fn mutate(&self, i: usize, mut_type: SwarBlockMutationType, from: Option<Vec<Swar>>) -> Option<SwarBlock> {
        match mut_type {
            SwarBlockMutationType::by_swar => {
                self.mutate_swar(i, from)
            },
            _ => {
                None
            }
        }
    }

    fn mutate_swar(&self, i: usize, from: Option<Vec<Swar>>) -> Option<SwarBlock> {
        let mut swars = self.to_swars().clone();
        let from_swr_blk = SwarBlock(from.unwrap());
        let mut rnd_swar = from_swr_blk.random_swar();

        let swar_mut_operators = rnd_swar.operators();
        let swar_mut_type = swar_mut_operators.choose(&mut rand::thread_rng());
        if let Some(swar_mut) = swar_mut_type {
            // mutations for Y in X Y Z -- X, Y, Z, X:Y, Y:X, Y:Z, Z:Y, X:X, Y:Y, Z:Z

            match swar_mut.to_string().as_str() {
                "simple" => {
                    std::mem::replace(&mut swars[i], rnd_swar);
                    println!("[simple] mutated: {}", SwarBlock(swars.clone()));

                    return Some(SwarBlock(swars));
                },
                "inc_beat" => {
                    let mut mut_rnd_swar = rnd_swar;
                    let inc_bt = mut_rnd_swar.beat_cnt + 1.0;
                    mut_rnd_swar.inc_beat_count(inc_bt);
                    std::mem::replace(&mut swars[i], mut_rnd_swar);
                    println!("[inc_beat] mutated: {}", SwarBlock(swars.clone()));

                    return Some(SwarBlock(swars));
                },
                "dec_beat" => {
                    let mut mut_rnd_swar = rnd_swar;
                    let mut dec_bt = mut_rnd_swar.beat_cnt;
                    if dec_bt >= 1.0 {
                        dec_bt = dec_bt/2.0;
                    }
                    mut_rnd_swar.dec_beat_count(dec_bt);
                    std::mem::replace(&mut swars[i], mut_rnd_swar);
                    println!("[dec_beat] mutated: {}", SwarBlock(swars.clone()));

                    return Some(SwarBlock(swars));
                },
                "share_beat" => {
                    // X Y Z -> X Y:_ Z
                    rnd_swar.set_beat_count(0.5);

                    // get another random swar from 'from'
                    let rnd_swar_latter = from_swr_blk.random_swar();

                    // now randomly insert/replace rnd_swar and rnd_swar_latter
                    let b = rand::thread_rng().gen_bool(0.5);
                    if b {
                        std::mem::replace(&mut swars[i], rnd_swar);
                        swars.insert(i+1, rnd_swar_latter);
                    } else {
                        swars.insert(i, rnd_swar_latter);
                        std::mem::replace(&mut swars[i+1], rnd_swar);
                    }

                    println!("[share_beat] mutated: {}", SwarBlock(swars.clone()));
                    return Some(SwarBlock(swars));
                },
                "kan_swar" => {
                    // X Y Z -> X _/_ Z
                    rnd_swar.set_beat_count(KAN_SWAR_BEAT_COUNT);

                    // get another random swar from 'from'
                    let rnd_swar_latter = from_swr_blk.random_swar();

                    // now randomly insert/replace rnd_swar and rnd_swar_latter
                    let b = rand::thread_rng().gen_bool((1.0 - KAN_SWAR_BEAT_COUNT) as f64);
                    if b {
                        std::mem::replace(&mut swars[i], rnd_swar);
                        swars.insert(i+1, rnd_swar_latter);
                    } else {
                        swars.insert(i, rnd_swar_latter);
                        std::mem::replace(&mut swars[i+1], rnd_swar);
                    }

                    println!("[kan_swar] mutated: {}", SwarBlock(swars.clone()));
                    return Some(SwarBlock(swars));
                },
                _ => { return None; }
            }
        }

        None
    }

    fn mutate_swar_duration(&self, _i: usize) -> Option<Swar> {
        None
    }
}
