use crate::raagas::swars::Swar;
use rand;
use rand::seq::SliceRandom;

use rand::prelude::ThreadRng;
use crate::raagas::sound::Pitch;
use crate::raagas::raag::raag::Raag;
use rand::Rng;
use crate::raagas::{SimpleRandomiser, PureRandomiser, Mutate, MutationType};
use crate::raagas::swarblock::SwarBlock;

pub fn index_swar(swars: &Vec<Swar>, swar: &Swar) -> Option<usize> {
    for (i, _swar) in swars.into_iter().enumerate() {
        if _swar.eq(&swar) {
            return Some(i);
        }
    }

    return None;
}

impl SimpleRandomiser for Raag {
    fn randomise(&self) -> Option<SwarBlock> {
        let mut rnd = rand::thread_rng();
        let base_swar_blk = self.pakad().as_ref().unwrap().to_swarblock();
        let pakad_blk = &base_swar_blk.0;
        let blk_len: i32 = pakad_blk.len() as i32;

        // exclude the left most and two right-most (as index start from 0)
        let i: i32 = rnd.gen_range(1, blk_len-2);
        let swar_picked = pakad_blk.get(i as usize).unwrap();
        let h = i - 1;
        let j = i + 2;

        println!("=> pakad: {}\n", base_swar_blk);
        println!("i = {}, swar: {}", i, swar_picked);
        if let Some(swars) = pakad_blk.get((h as usize)..(j as usize)) {
            let swars_vec = swars.to_vec();
            println!("blk: {}", SwarBlock(swars_vec.clone()));
            if self.is_ascending(&swars_vec) {
                println!("\n=> swars in ascending ...");
                // use swars around 'i'th swar from aroha.
                if let Some(aroha_swars) = self.aroha_swars_by_context(swar_picked) {
                    println!("aroha: {:?}", SwarBlock(aroha_swars.clone()));
                    let mut_pakad: SwarBlock = base_swar_blk.mutate(
                        i as usize,
                        MutationType::by_swar,
                        Some(aroha_swars)
                    );
                    return Some(mut_pakad);
                }
            }

            if self.is_descending(&swars_vec) {
                println!("\n=> swars in descending ...");
                // use swars around 'i'th swar from aroha.
                if let Some(avroha_swars) = self.avroha_swars_by_context(swar_picked) {
                    println!("avroha: {}", SwarBlock(avroha_swars.clone()));
                    let mut_pakad: SwarBlock = base_swar_blk.mutate(
                        i as usize, MutationType::by_swar, Some(avroha_swars));
                    return Some(mut_pakad);
                }
            }

            // swars (R S G) -- neither asc or descending
            // so try: ith swar S -> S:S, or  R:S or S:G or R or G
            println!("\n=> swars neither asecending or descending ...");
            let mut_pakad: SwarBlock = base_swar_blk.mutate(
                i as usize,
                MutationType::by_swar,
                Some(swars_vec)
            );
            return Some(mut_pakad);
        }

        return None
    }
}

impl PureRandomiser for Raag {
    fn randomise(&self, n_swars: usize) -> Result<Vec<Swar>, String> {
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

