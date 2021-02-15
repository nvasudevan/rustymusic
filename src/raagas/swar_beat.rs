use crate::raagas::swars::Swar;
use std::fmt;
use std::fmt::Formatter;
use rand;
use rand::seq::SliceRandom;

use crate::raagas::constants::KAN_SWAR_BEAT_COUNT;

// maps to 1 beat in a rhythm cycle
#[derive(Debug, Clone)]
pub struct SwarBeat {
    // possible combinations:
    // S, S:S (or :S or S:), S:S:S:S
    pub swars: Vec<Swar>,
}

impl SwarBeat {
    pub fn new(swars: Vec<Swar>) -> Self {
        SwarBeat {
            swars
        }
    }

    pub fn len(&self) -> usize {
        self.swars.len()
    }

    pub fn random_swar(&self) -> Swar {
        let mut rnd = rand::thread_rng();
        let rnd_swar = self.swars.choose(&mut rnd);

        let swar = rnd_swar.unwrap();

        swar.clone()
    }

    pub fn replace_swar(&mut self, index: usize, swar: Swar) {
        let swars: &mut Vec<Swar> = self.swars.as_mut();
        let _ = std::mem::replace(&mut swars[index],swar);
    }

    pub fn insert_swar(&mut self, index: usize, swar: Swar) {
        let swars: &mut Vec<Swar> = self.swars.as_mut();
        swars.insert(index, swar);
    }
}

impl fmt::Display for SwarBeat {
    // there are four options: S, S:S, S:S:S:S, S/G
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let first_swar = self.swars.first().unwrap();
        let mut s = format!("{}", first_swar);

        let rest_swars = self.swars.get(1..);
        if let Some(more_swars) = rest_swars {
            let mut prev_bt_cnt = first_swar.beat_cnt;
            for swar in more_swars {
                if (swar.beat_cnt == 0.25) || (swar.beat_cnt == 0.50) {
                    s = format!("{}:{}", s, swar);
                } else {
                    if prev_bt_cnt == KAN_SWAR_BEAT_COUNT {
                        s = format!("{}/{}", s, swar);
                    } else {
                        s = format!("{} {}", s, swar);
                    }
                }
                prev_bt_cnt = swar.beat_cnt;
            }
        }

        write!(f, "{}", s)
    }
}