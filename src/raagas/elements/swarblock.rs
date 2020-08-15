use crate::raagas::elements::elements::{AudioDevice, Melody, Swar};
use crate::raagas::utils;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

impl SwarBlock {
    // fn count_swars(&self) -> usize {
    //     self.0.len()
    // }
    //
    // fn n_swars(&self, n: usize) -> Option<SwarBlock> {
    //     let _swars: Vec<Swar> = (&self.0).clone();
    //     let swars = &_swars[0..n];
    //     Some(SwarBlock(Vec::from(swars)))
    // }
}

impl Melody for SwarBlock {
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        n: i8,
    ) {
        for _ in 0..n {
            for bt in &self.0 {
                print!("{} ", bt);
                utils::io_flush();
                // play_swar_with_taal(&dev, &bt, Some(&beat_src));
                bt.play(&dev, beat_src.clone(), 1);
            }
        }
    }
}
