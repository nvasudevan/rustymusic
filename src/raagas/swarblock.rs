use crate::raagas::swars::{Swar, BeatSrc};
use std::fmt;
use std::iter::FromIterator;
use crate::raagas::sound::{AudioDevice, TimedSink};
use rodio::PlayError;
use crate::raagas::{Mutate, utils, MutationType};
use rand::seq::SliceRandom;
use crate::raagas::constants::BPS;

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

    pub fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
    ) {
        //
        match self.build_sink(&None, &dev, vol) {
            Ok(tsinks) => {
                for tsink in tsinks {
                    tsink.sink.play();
                    utils::delay(tsink.duration * BPS);
                    tsink.sink.stop();
                }
            },
            _ => {}
        }
    }

    pub fn play_rt(&self, dev: &AudioDevice, vol: f32) {
        for bt in &self.0 {
            let tsink = bt.build_sink(&None, &dev, vol).unwrap();
            tsink.sink.play();
            utils::delay(tsink.duration * BPS);
            tsink.sink.stop();
        }

    }
}
impl fmt::Display for SwarBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for swar in &self.0 {
            s = format!("{} {}", s, swar);
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
    pub fn build_sink(&self,
                      beat_src: &Option<BeatSrc>,
                      dev: &AudioDevice,
                      vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        let mut sinks: Vec<TimedSink> = Vec::new();
        for blk in &self.0 {
            println!("blk: {:?}", blk);
            let mut blk_sinks = blk.build_sink(&beat_src, &dev, vol)?;
            sinks.append(&mut blk_sinks);
        }

        Ok(sinks)
    }

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
}

impl Mutate for SwarBlocks {
    fn mutate(&self, i: usize, mut_type: MutationType, from: Option<Vec<Swar>>) -> SwarBlock {
        self.to_swarblock().mutate(i, mut_type, from)
    }

    fn mutate_swar(&self, i: usize, from: Vec<Swar>) -> SwarBlock {
        unimplemented!()
    }

    fn muate_swar_duration(&self, i: usize) -> SwarBlock {
        unimplemented!()
    }
}

impl Mutate for SwarBlock {
    fn mutate(&self, i: usize, mut_type: MutationType, from: Option<Vec<Swar>>) -> SwarBlock {
        let mut swars = self.to_swars().clone();
        println!("i={}, swars: {}", i, SwarBlock(swars.clone()));
        let mut rnd = rand::thread_rng();
        if let Some(rnd_swar) = from.unwrap().choose(&mut rnd) {
            println!("rnd_swar: {}", rnd_swar);
            std::mem::replace(&mut swars[i], rnd_swar.clone());
        }
        println!("mut swars: {}", SwarBlock(swars.clone()));

        // other mutations for Y in X Y Z -- X:Y, Y:Z, X, Y, X:X, Z:Z

        SwarBlock(swars)
    }

    fn mutate_swar(&self, i: usize, from: Vec<Swar>) -> SwarBlock {
        unimplemented!()
    }

    fn muate_swar_duration(&self, i: usize) -> SwarBlock {
        unimplemented!()
    }
}
