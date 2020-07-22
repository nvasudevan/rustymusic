use rand;
use crate::raagas::elements::Swar;
use crate::SWARS;
use crate::raagas::elements::{Pitch, BPM};
use rand::seq::SliceRandom;


pub fn randomiser(n: u32) -> Vec<Swar> {
    let swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
    let mut rnd = rand::thread_rng();
    let beats: Vec<Swar> = swars.choose_multiple(&mut rnd, n as usize)
                                .map(|x| x.to_string())
                                .map(|s|
                                    Swar {
                                        swar: Some(Pitch::new(s)),
                                        beat_cnt: 1
                                    }
                                )
                                .collect();

    beats
}
