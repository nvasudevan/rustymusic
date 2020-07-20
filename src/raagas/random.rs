use rand;
use rand::{Rng, random};
use crate::raagas::elements::Beat;
use crate::SWARS;
use crate::raagas::swars;

fn rand_next() -> Option<u8> {
    let mut rnd = rand::thread_rng();
    let n: u8 = rnd.gen();

    Some(n)
}

pub fn randomiser(n: u32) -> Vec<Beat> {
    let mut beats: Vec<Beat> = Vec::new();
    let swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
    for i in 0..n {
        let r = rand_next().unwrap();
        let x = r % 8;
        let swar: String = swars.get(x as usize).unwrap().to_string();
        beats.push(
            Beat { swar: Some(swars::Pitch::new(swar)),
                long: swars::BASE_SWAR_INTERVAL }
        );
    }

    beats
}

