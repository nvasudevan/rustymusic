use std::{fmt, io, fs};

use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration, SineWave};
use crate::raagas::sound::{Pitch, AudioDevice, TimedSink};
use rodio::{Sink, Source, PlayError};


use std::io::Write;



pub type BeatSrc = Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>;

#[derive(Debug, Clone)]
pub struct Swar {
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

    pub fn pitch(&self) -> Option<Pitch> {
        self.pitch.clone()
    }

    pub fn beat_count(&self) -> f32 {
        self.beat_cnt
    }

    pub(crate) fn build_sink(&self, beat_src: &Option<BeatSrc>, dev: &AudioDevice, vol: f32) -> Result<TimedSink, PlayError> {
        let sink = Sink::try_new(&dev.out_stream_handle)?;
        match beat_src.clone() {
            Some(src) => {
                match self.pitch.as_ref() {
                    // play swar with taal
                    Some(p) => {
                        let sinew = SineWave::from(p.to_owned());
                        sink.append(sinew);
                        // sink.append(sinew.mix(Pitch::from_swar("M")));
                        // sink.append(src.mix(sinew));
                    }
                    _ => {
                        // play taal
                        sink.append(src);
                    }
                }
            }
            _ => {
                // play swar
                if let Some(p)  = self.pitch.as_ref() {
                    print!("{} ", p);
                    io::stdout().flush();
                    let sinew = SineWave::from(p.to_owned());
                    // sink.append(sinew);
                    sink.append(sinew.mix(Pitch::from_swar("M")));
                }
            }
        }
        sink.set_volume(vol);
        Ok(TimedSink::new(sink, self.beat_cnt))
    }
}

impl PartialEq for Swar {
    fn eq(&self, other: &Self) -> bool {
        if self.pitch.as_ref().unwrap().hertz().unwrap().freq() == other.pitch.as_ref().unwrap().hertz().unwrap().freq() {
            return true;
        }

        return false;
    }
}

impl fmt::Display for Swar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _cnt = self.beat_cnt as usize;
        let dash = match _cnt {
            1 => String::from(" "),
            2..=8 => (0..(_cnt - 1)).map(|_| " - ").collect::<String>(),
            _ => String::new(),
        };

        let _s = match &self.pitch {
            Some(sw) => format!("{}{}", sw, dash),
            _ => String::new(),
        };
        write!(f, "{}", _s)
    }
}

