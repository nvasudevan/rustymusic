use crate::raagas::elements::elements::{Pitch, Swar};
use crate::SWARS;
use rand;
use rand::seq::SliceRandom;

pub fn randomiser(n: usize) -> Vec<Swar> {
    let swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
    let mut rnd = rand::thread_rng();
    let beats: Vec<Swar> = swars
        .choose_multiple(&mut rnd, n as usize)
        .map(|x| x.to_string())
        .map(|s| Swar {
            pitch: Some(Pitch::new(s)),
            beat_cnt: 2.0,
        })
        .collect();

    beats
}
