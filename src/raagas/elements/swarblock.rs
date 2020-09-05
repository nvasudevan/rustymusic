use crate::raagas::elements::elements::{AudioDevice, Melody, Swar};
use crate::raagas::utils;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

impl SwarBlock {
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for sw in &self.0 {
            swars.push(sw.clone());
        }
        swars
    }
}

impl FromIterator<usize> for SwarBlock {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        let mut _blk: Vec<Swar> = Vec::new();
        for _ in iter {
            _blk.push(Swar { pitch: None, beat_cnt: 1.0 });
        }

        SwarBlock(_blk)
    }
}

impl Melody for SwarBlock {
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        _mix: bool,
        n: i8,
    ) {
        for _ in 0..n {
            let mut prev_sw_bt: f32 = 1.0;
            for bt in &self.0 {
                if (prev_sw_bt < 1.0) && (bt.beat_cnt > 1.0) {
                    print!(" {}", bt);
                } else {
                    print!("{}", bt);
                }
                utils::io_flush();
                bt.play(&dev, beat_src.clone(), false, 1);
                prev_sw_bt = bt.beat_cnt;
            }
        }
    }
}
