use crate::raagas::swars::{Swar, SwarBlock};
use rand;
use rand::seq::SliceRandom;

use rand::prelude::ThreadRng;
use crate::raagas::raag::{raag, Randomiser};
use crate::raagas::sound::Pitch;
use crate::raagas::raag::raag::Raag;
use rand::Rng;

fn index_swar(swars: &Vec<Swar>, swar: &Swar) -> Option<usize> {
    for (i, _swar) in swars.into_iter().enumerate() {
        if _swar.eq(&swar) {
            return Some(i);
        }
    }

    return None;
}

impl Randomiser for Raag {
    // we will do only one mutation on the swarblock.
    // randomly pick an element and change it a neighbour swar from aroha/avroha
    fn randomise_swarblocks(&self, swarblock: &SwarBlock) -> SwarBlock {
        let mut rnd = rand::thread_rng();
        let i = rnd.gen_range(0, swarblock.0.len());
        let i_swar: &Swar = swarblock.0.get(i).unwrap();
        println!("i: {}, swar: {}", i, i_swar);

        let aroha_swars =  self.aroha().as_ref().unwrap().to_swars();
        println!("aroha_swars: {:?}", aroha_swars);
        let aroha_i = index_swar(&aroha_swars, &i_swar).unwrap();
        println!("aroha_i: {}", aroha_i);
        println!("i-1: {}, i+1: {}", &aroha_swars.get(aroha_i-1).unwrap(), &aroha_swars.get(aroha_i+1).unwrap());


        let avroha_swars =  self.avroha().as_ref().unwrap().to_swars();
        println!("avroha_swars: {:?}", avroha_swars);
        let avroha_i = index_swar(&avroha_swars, &i_swar).unwrap();
        println!("avroha_i: {}", avroha_i);
        println!("i-1: {}, i+1: {}", &avroha_swars.get(avroha_i-1).unwrap(), &avroha_swars.get(avroha_i+1).unwrap());

        let swars: Vec<Swar> = Vec::new();
        SwarBlock(swars)
    }

    fn random_swars(&self, n_swars: usize) -> Result<Vec<Swar>, String> {
        let mut rnd = rand::thread_rng();
        let aroha = self.aroha().as_ref().unwrap();
        let avroha = self.avroha().as_ref().unwrap();
        let mut _swars: Vec<Swar> = Vec::new();

        // always start with S
        for blk in &aroha.0 {
            _swars.append(&mut blk.to_swars());
        }

        for blk in &avroha.0 {
            _swars.append(&mut blk.to_swars());
        }

        let mut swars: Vec<Swar> = Vec::new();
        swars.push(Swar::new(Pitch::new("S".to_string()), 3.0));
        // choose swars in aroha and some swars in avroha
        // as the swars are not the same between the two.

        let _inds: Vec<usize> = get_rnd_monotonic(&mut rnd, _swars.len(), n_swars);
        for i in _inds {
            if let Some(sw) = _swars.get(i) {
                if let Some(p) = sw.pitch.as_ref() {
                    swars.push(Swar::new(p.clone(), 2.0));
                } else {
                    return Err(format!("Pitch can't be empty!"));
                }
            } else {
                return Err(format!(
                    "Can't retrieve swar at index {} in aroha/avroha!",
                    i
                ));
            }
        }

        // end with S
        swars.push(Swar::new(Pitch::new("S".to_string()), 3.0));


        Ok(swars)
    }
}

fn get_rnd_monotonic(mut rnd: &mut ThreadRng, max: usize, n: usize) -> Vec<usize> {
    let _v: Vec<usize> = (0..max).collect();
    let mut _rnd: Vec<usize> = _v.choose_multiple(&mut rnd, n).map(|i| *i).collect();
    _rnd.sort();

    _rnd
}

