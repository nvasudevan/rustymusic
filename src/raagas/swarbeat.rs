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

    /// Returns a new `SwarBeat` based on `Swars`.
    pub fn new(swars: Vec<Swar>) -> Self {
        SwarBeat {
            swars
        }
    }

    /// Calculates the length (i.e. number of swars) contained in it
    pub fn len(&self) -> usize {
        self.swars.len()
    }

    /// Derive the lower octave equivalent of a swarbeat
    pub fn lower(&self) -> SwarBeat {
        let mut lower_swars = Vec::<Swar>::new();
        for sw in &self.swars {
            let lower_swar = sw.lower();
            lower_swars.push(lower_swar);
        }

        SwarBeat::new(lower_swars)
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

    /// Find the first swar with a pitch traversing back from index `from`
    pub fn swar_index_reverse_from(&self, from: usize) -> Option<(usize, &Swar)> {
        for j in (0..from+1).rev() {
            println!("  j: {}", j);
            if let Some(swar) = self.swars.get(j) {
                println!("  swar: {}", swar);
                if let Some(_) = &swar.pitch {
                    return Some((j, &swar));
                }
            }
        }

        None
    }

    /// Find the first swar with a pitch traversing back from the rightmost swar
    pub fn swar_index_reverse(&self) -> Option<(usize, &Swar)> {
        if self.len() > 0 {
            return self.swar_index_reverse_from(self.len());
        }

        None
    }

    /// Find the first swar with a pitch traversing forward from index `from`
    pub fn swar_index_forward_from(&self, from: usize) -> Option<(usize, &Swar)> {
        for j in from..self.len() {
            println!("  j: {}", j);
            if let Some(swar) = self.swars.get(j) {
                println!("  swar: {}", swar);
                if let Some(_) = &swar.pitch {
                    return Some((j, &swar));
                }
            }
        }

        None
    }

    /// Find the first swar with a pitch traversing forward from leftmost swar
    pub fn swar_index_forward(&self) -> Option<(usize, &Swar)> {
        if self.len() > 0 {
            return self.swar_index_forward_from(0);
        }

        None
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

#[cfg(test)]
mod tests {
    use crate::raagas::sound::Pitch;
    use crate::raagas::swars::Swar;
    use crate::raagas::swarbeat::SwarBeat;
    use crate::raagas::constants::KAN_SWAR_BEAT_COUNT;

    /// test a swarbeat with a pair of swars
    #[test]
    fn test_swarbeat_repr_double() {
        let sa_pitch = Pitch::new("S".to_string());
        let ma_pitch = Pitch::new("M".to_string());
        let sa = Swar::new(sa_pitch, 0.5);
        let ma = Swar::new(ma_pitch, 0.5);
        let mut swars = Vec::<Swar>::new();
        swars.push(sa);
        swars.push(ma);
        let sw_bt: SwarBeat = SwarBeat::new(swars);
        assert_eq!(sw_bt.to_string(), "S:M");
    }

    /// test a swarbeat representation with quadruple swars
    #[test]
    fn test_swarbeat_repr_quadruple() {
        let sa_pitch = Pitch::new("S".to_string());
        let re_pitch = Pitch::new("R".to_string());
        let ma_pitch = Pitch::new("M".to_string());
        let pa_pitch = Pitch::new("P".to_string());
        let sa = Swar::new(sa_pitch, 0.25);
        let re = Swar::new(re_pitch, 0.25);
        let ma = Swar::new(ma_pitch, 0.25);
        let pa = Swar::new(pa_pitch, 0.25);
        let mut swars = Vec::<Swar>::new();
        swars.push(sa);
        swars.push(re);
        swars.push(ma);
        swars.push(pa);
        let sw_bt: SwarBeat = SwarBeat::new(swars);
        assert_eq!(sw_bt.to_string(), "S:R:M:P");
    }

    /// test a swarbeat representation with kan swar
    #[test]
    fn test_swarbeat_repr_kan_swar() {
        let ni_pitch = Pitch::new(".N".to_string());
        let sa_pitch = Pitch::new("S".to_string());
        let ni = Swar::new(ni_pitch, KAN_SWAR_BEAT_COUNT);
        let sa = Swar::new(sa_pitch, 1.0 - KAN_SWAR_BEAT_COUNT);
        let mut swars = Vec::<Swar>::new();
        swars.push(ni);
        swars.push(sa);
        let sw_bt: SwarBeat = SwarBeat::new(swars);
        assert_eq!(sw_bt.to_string(), ".N/S");
    }

    /// test swarbeat representation with half empty beat
    #[test]
    fn test_swarbeat_repr_half_empty_beat() {
        let empty_pitch = Pitch::new("".to_string());
        let sa_pitch = Pitch::new("S".to_string());
        let empty = Swar::new(empty_pitch, 0.5);
        let sa = Swar::new(sa_pitch, 0.5);
        let mut swars = Vec::<Swar>::new();
        swars.push(empty);
        swars.push(sa);
        let sw_bt: SwarBeat = SwarBeat::new(swars);
        assert_eq!(sw_bt.to_string(), ":S");
    }
}