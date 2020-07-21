use rand;
use crate::raagas::elements::Beat;
use crate::SWARS;
use crate::raagas::elements::{Pitch, BASE_SWAR_INTERVAL};
use rand::seq::SliceRandom;


pub fn randomiser(n: u32) -> Vec<Beat> {
    let swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
    let mut rnd = rand::thread_rng();
    let beats: Vec<Beat> = swars.choose_multiple(&mut rnd, n as usize)
                                .map(|x| x.to_string())
                                .map(|s|
                                    Beat {
                                        swar: Some(Pitch::new(s)),
                                        long: BASE_SWAR_INTERVAL
                                    }
                                )
                                .collect();

    beats
}
