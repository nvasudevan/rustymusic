use std::{fmt, io, fs};

use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use crate::raagas::sound::{Pitch, AudioDevice};
use rodio::{Sink, Source, PlayError};

use crate::raagas::{Mutate, MutationOperators};
use crate::raagas::swarblock::SwarInSwarBlock;
use rand::seq::SliceRandom;

pub type BeatSrc = Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>;

#[derive(Debug, Clone)]
pub struct Swar {
    // A blank swar is when there is nothing to play, so a pause
    pub pitch: Option<Pitch>,
    pub beat_cnt: f32,
}

// impl for Swar
impl Swar {
    pub fn new(pitch: Pitch, beat_cnt: f32) -> Swar {
        Swar {
            pitch: Some(pitch),
            beat_cnt,
        }
    }

    pub fn empty(beat_cnt: f32) -> Self {
        Swar {
            pitch: None,
            beat_cnt
        }
    }

    pub fn pitch(&self) -> Option<Pitch> {
        self.pitch.clone()
    }

    pub fn beat_count(&self) -> f32 {
        self.beat_cnt
    }

    pub fn set_beat_count(&mut self, bt_cnt: f32) {
        self.beat_cnt = bt_cnt;
    }

    pub fn freq(&self) -> f64 {
        self.pitch.as_ref().unwrap().hertz().unwrap().freq()
    }

    /// increment the swar beat count by inc
    pub(crate) fn inc_beat_count(&mut self, inc: f32) {
        self.beat_cnt += inc;
    }

    /// decrement the swar beat count by dec
    pub(crate) fn dec_beat_count(&mut self, dec: f32) {
        self.beat_cnt -= dec;
    }

    pub(crate) fn build_sink(&self,
                             beat_src: &Option<BeatSrc>,
                             dev: &AudioDevice) -> Result<Option<Sink>, PlayError> {
        let sink = Sink::try_new(&dev.out_stream_handle)?;
        sink.set_volume(dev.vol());
        match beat_src {
            Some(_) => {
                if let Some(p) = self.pitch.as_ref() {
                    // play swar with taal
                    // let sinew = SineWave::from(p.to_owned());
                    if let Some(sinew) = p.to_sinewave() {
                        sink.append(sinew);
                        // sink.append(sinew.mix(Pitch::from_swar("M")));
                        // sink.append(src.mix(sinew));
                        return Ok(Some(sink));
                    }
                }
            }
            _ => {
                // play swar
                if let Some(p)  = self.pitch.as_ref() {
                    if let Some(sinew) = p.to_sinewave() {
                        // sink.append(sinew);
                        sink.append(sinew.mix(Pitch::from_swar("M")));
                        return Ok(Some(sink));
                    }
                }
            }
        }

        Ok(None)
    }
}

impl PartialEq for Swar {
    fn eq(&self, other: &Self) -> bool {
        let my_freq= self.pitch.as_ref().unwrap().hertz().unwrap().freq();
        let other_freq= other.pitch.as_ref().unwrap().hertz().unwrap().freq();
        if my_freq == other_freq {
            return true;
        }

        return false;
    }
}

impl fmt::Display for Swar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _s = match &self.pitch {
            Some(p) => format!("{}", p),
            _ => String::new(),
        };
        write!(f, "{}", _s)
    }
}

impl MutationOperators for Swar {
    fn operators(&self) -> Vec<&str> {
        vec![
            "simple", "inc_beat", "dec_beat", "share_beat", "kan_swar"
        ]
    }

    fn random_mutation_operator(&self) -> String {
         let swar_mut_operators = &self.operators();
         let swar_mut_type = swar_mut_operators.choose(&mut rand::thread_rng()).unwrap();

        swar_mut_type.to_string()

    }
}

impl Mutate for Swar {
    fn mutate(&self, _index: &SwarInSwarBlock, _from: Vec<&Swar>) -> Self {
        unimplemented!()
    }

    fn mutate_swar(&self, _index: &SwarInSwarBlock, _from: Vec<&Swar>) -> Self {
        unimplemented!()
    }

    fn mutate_swar_duration(&self, _i: usize) -> Option<Swar> {
        let mut mut_swar = self.clone();
        let beat_durations: Vec<f32> = vec![0.5, 1.0, 2.0, 3.0];
        let mut rnd = rand::thread_rng();
        mut_swar.beat_cnt = *beat_durations.choose(&mut rnd).unwrap_or_else(|| &(1.0 as f32));

        Some(mut_swar)
    }
}

/// tests on swarbeats and swars
#[cfg(test)]
mod tests {
    use crate::raagas::sound::{Hertz, Pitch};
    use crate::raagas::swars::Swar;

    /// test S is set to C#, base pitch
    #[test]
    fn test_base_swar_is_sa() {
        let base_hz = Hertz::new(277.18, "C#".to_string());
        let sa_pitch = Pitch::new("S".to_string());
        let sa = Swar::new(sa_pitch, 1.0);
        assert_eq!(sa.pitch.unwrap().hertz().unwrap(), base_hz);
    }

    /// test swar with single beat
    #[test]
    fn test_swar_repr_single_beat() {
        // test string version of swar
        let sa_pitch = Pitch::new("S".to_string());
        let sa = Swar::new(sa_pitch, 1.0);
        assert_eq!(sa.to_string(), "S");
    }
}
