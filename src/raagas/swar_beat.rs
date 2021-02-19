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

    pub fn increment_swar_at(&mut self, index: usize, beat_count_inc: f32) {
         if let Some(swar) =  self.swars.get_mut(index) {
              swar.inc_beat_count(beat_count_inc);
         }
    }
}

impl fmt::Display for SwarBeat {
    // there are four options: S, S:S, S:S:S:S, S/G
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // if there no swars in that swarbeat, it has a "-"? meaning it is an extension
        // of previous swar?
        let mut s = String::from("-");
        match self.swars.first() {
             Some(first_swar) => {
                 let rest_swars = self.swars.get(1..).unwrap();
                 match rest_swars.len() {
                     0 => {
                         // M:M -:P
                         // the second swarbeat has only one swar (0.5 beat) but rendered as 'P'
                         // so we count the number of beats, and if < 1.0 then we add a ':'
                         if first_swar.beat_cnt < 1.0 {
                             s = format!("-:{}", first_swar);
                         } else {
                             s = format!("{}", first_swar);
                         }
                     },
                     _ => {
                         let mut prev_bt_cnt = first_swar.beat_cnt;
                         s = format!("{}", first_swar);
                         for swar in rest_swars {
                             // when we have the rest of the swars with fraction beat
                             if prev_bt_cnt == KAN_SWAR_BEAT_COUNT {
                                 s = format!("{}/{}", s, swar);
                             } else {
                                 s = format!("{}:{}", s, swar);
                             }
                             prev_bt_cnt = swar.beat_cnt;
                         }
                     }
                 }
             },
            _ => {}
        }

        write!(f, "{}", s)
    }
}