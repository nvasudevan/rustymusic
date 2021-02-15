use crate::raagas::swars::Swar;
use rand;
use rand::seq::SliceRandom;

use rand::prelude::ThreadRng;
use crate::raagas::sound::Pitch;
use crate::raagas::raag::raag::Raag;
use crate::raagas::{SimpleRandomiser, PureRandomiser, Mutate};
use crate::raagas::swarblock::SwarBlocks;

impl SimpleRandomiser for Raag {
    fn randomise(&self, src_blks: &SwarBlocks) -> SwarBlocks {
        if let Some(rnd_swar_ind) = src_blks.random_swar_index() {
            if let Some(context_swars) = src_blks.adjacent_swars(&rnd_swar_ind) {
               println!("swars: {:?}", context_swars);
                if self.is_ascending(&context_swars) {
                    println!("\n=> swars in ascending ...");
                    let swar_picked = &rnd_swar_ind.swar;
                    if let Some(aroha_swars) = self.aroha_swars_by_context(&swar_picked) {
                        println!("aroha: {:?}", aroha_swars);
                        let mut_src_blk = src_blks.mutate(
                            &rnd_swar_ind,
                            aroha_swars
                        );
                        return mut_src_blk;
                    }
                }

                if self.is_descending(&context_swars) {
                    println!("\n=> swars in descending ...");
                    let swar_picked = &rnd_swar_ind.swar;
                    if let Some(avroha_swars) = self.avroha_swars_by_context(swar_picked) {
                        println!("avroha: {:?}", avroha_swars);
                        let mut_src_blk = src_blks.mutate(
                            &rnd_swar_ind,
                            avroha_swars);
                        return mut_src_blk;
                    }
                }

                // swars (R S G) -- neither asc or descending
                // so try: ith swar S -> S:S, or  R:S or S:G or R or G
                println!("\n=> swars neither asecending or descending ...");
                let mut_src_blk = src_blks.mutate(
                    &rnd_swar_ind,
                    context_swars
                );
                return mut_src_blk;
            }
        }

        // pointless -- what should happen?
        src_blks.clone()
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

