use crate::raagas::swars::Swar;
use rand;
use rand::seq::SliceRandom;

use rand::prelude::ThreadRng;
use crate::raagas::raag::raag;

fn get_rnd_monotonic(mut rnd: &mut ThreadRng, max: usize, n: usize) -> Vec<usize> {
    let _v: Vec<usize> = (0..max).collect();
    let mut _rnd: Vec<usize> = _v.choose_multiple(&mut rnd, n).map(|i| *i).collect();
    _rnd.sort();

    _rnd
}

pub fn randomiser(raag: &raag::Raag, n: usize) -> Result<Vec<Swar>, String> {
    let mut rnd = rand::thread_rng();
    let aroha = raag.aroha().as_ref().unwrap();
    let avroha = raag.avroha().as_ref().unwrap();
    let mut _swars: Vec<Swar> = Vec::new();
    for blk in &aroha.0 {
        _swars.append(&mut blk.to_swars());
    }

    for blk in &avroha.0 {
        _swars.append(&mut blk.to_swars());
    }

    let mut swars: Vec<Swar> = Vec::new();
    // choose swars in aroha and some swars in avroha
    // as the swars are not the same between the two.

    let _inds: Vec<usize> = get_rnd_monotonic(&mut rnd, _swars.len(), n);
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

    Ok(swars)
}
