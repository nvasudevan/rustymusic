use crate::raagas::swars::{Swar, BeatSrc};
use std::fmt;
use std::iter::FromIterator;
use crate::raagas::sound::{AudioDevice, TimedSink};
use rodio::PlayError;
use crate::raagas::Mutate;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

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

    pub fn to_swarblock(&self) -> SwarBlock {
        SwarBlock(self.to_swars())
    }
}

impl Mutate for SwarBlocks {
    fn mutate(&self, i: usize, from: Vec<Swar>) -> SwarBlock {
        self.to_swarblock().mutate(i, from)
    }
}

impl Mutate for SwarBlock {
    fn mutate(&self, i: usize, from: Vec<Swar>) -> SwarBlock {
        let mut swars = self.to_swars().clone();
        println!("i={}, swars: {}", i, SwarBlock(swars.clone()));
        let mut rnd = rand::thread_rng();
        if let Some(rnd_swar) = from.choose(&mut rnd) {
            println!("rnd_swar: {}", rnd_swar);
            std::mem::replace(&mut swars[i], rnd_swar.clone());
            // swars.remove(i);
            // swars.insert(i, rnd_swar.clone());
        }
        println!("mut swars: {}", SwarBlock(swars.clone()));

        // other mutations for Y in X Y Z -- X:Y, Y:Z, X, Y, X:X, Z:Z

        SwarBlock(swars)
    }
}
