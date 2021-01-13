use std::{fmt, io, fs};
use crate::raagas::utils;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration, SineWave};
use crate::raagas::physics::{Pitch, AudioDevice, TimedSink};
use rodio::{Sink, Source, PlayError};
use crate::raagas::constants::BPS;
use std::iter::FromIterator;
use std::io::sink;

pub type BeatSrc = Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Swar {
    pub pitch: Option<Pitch>,
    pub beat_cnt: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

#[derive(Debug, Clone)]
pub struct SwarBlocks(pub Vec<SwarBlock>);

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

    fn build_sink(&self, beat_src: &Option<BeatSrc>, dev: &AudioDevice, vol: f32) -> Result<TimedSink, PlayError> {
        let sink = Sink::try_new(&dev.out_stream_handle)?;
        match beat_src.clone() {
            Some(src) => {
                match self.pitch.as_ref() {
                    // play swar with taal
                    Some(p) => {
                        let sinew = SineWave::from(p.to_owned());
                        sink.append(src.mix(sinew));
                    }
                    _ => {
                        // play taal
                        sink.append(src);
                    }
                }
            }
            _ => {
                // play swar
                match self.pitch.as_ref() {
                    Some(p) => {
                        let sinew = SineWave::from(p.to_owned());
                        sink.append(sinew);
                        // sink.append(sinew.mix(Pitch::from_swar("M")));
                    }
                    _ => {}
                }
            }
        }
        sink.set_volume(vol);
        Ok(TimedSink::new(sink, self.beat_cnt))
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

// impl for SwarBlock
impl SwarBlock {
    pub fn to_swars(&self) -> Vec<Swar> {
        let mut swars: Vec<Swar> = Vec::new();
        for sw in &self.0 {
            swars.push(sw.clone());
        }
        swars
    }

    pub fn no_beats(&self) -> usize {
        let mut cnt: usize = 0;
        for sw in &self.0 {
            cnt = cnt + sw.beat_cnt as usize;
        }

        cnt
    }

    pub fn build_sink(&self,
                       beat_src: &Option<BeatSrc>,
                       dev: &AudioDevice,
                       vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        let mut sinks: Vec<TimedSink> = Vec::new();
        for bt in &self.0 {
            let bt_sink = bt.build_sink(&beat_src, &dev, vol)?;
            sinks.push(bt_sink);
        }

        Ok(sinks)
    }
}

impl FromIterator<usize> for SwarBlock {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        let mut _blk: Vec<Swar> = Vec::new();
        for _ in iter {
            _blk.push(Swar {
                pitch: None,
                beat_cnt: 1.0,
            });
        }

        SwarBlock(_blk)
    }
}

impl SwarBlocks {
    pub fn build_sink(&self,
                      beat_src: &Option<BeatSrc>,
                      dev: &AudioDevice,
                      vol: f32) -> Result<Vec<TimedSink>, PlayError> {
        let mut sinks: Vec<TimedSink> = Vec::new();
        for blk in &self.0 {
            println!("blk: {:?}", blk);
            let mut blk_sinks = blk.build_sink(&beat_src, &dev, vol)?;
            sinks.append(&mut blk_sinks);
        }

        Ok(sinks)
    }
}
