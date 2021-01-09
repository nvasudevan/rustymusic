use std::{fmt,io,fs};
use crate::raagas::utils;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration, SineWave};
use crate::raagas::physics::{Pitch, AudioDevice};
use rodio::{Sink, Source};
use crate::raagas::constants::BPS;
use std::iter::FromIterator;
use std::io::sink;

pub trait Melody {
    fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        beat_src: Option<Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>>,
        mix: bool,
        n: i8,
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct Swar {
    pub pitch: Option<Pitch>,
    pub beat_cnt: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwarBlock(pub Vec<Swar>);

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

impl Melody for Swar {
    fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        beat_src: Option<Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>>,
        _mix: bool,
        _n: i8,
    ) {
        match Sink::try_new(&dev.out_stream_handle) {
            Ok(sink) => {
                match beat_src {
                    Some(src) => {
                        match &self.pitch {
                            // play swar with taal
                            Some(p) => {
                                let sinew = SineWave::from(p.to_owned());
                                sink.append(src.mix(sinew));
                                // let sa = Pitch::default().hertz().unwrap().freq() as u32;
                                // sink.append(src.mix(SineWave::new(sa)));
                            }
                            _ => {
                                // play taal
                                sink.append(src);
                            }
                        }
                    }
                    _ => {
                        // play swar
                        match &self.pitch {
                            Some(p) => {
                                let sinew = SineWave::from(p.to_owned());
                                sink.append(sinew);
                            }
                            _ => {}
                        }
                    }
                }
                sink.set_volume(vol);
                sink.play();
                utils::delay(self.beat_cnt * BPS);
                sink.stop();
            },
            Err(e) => {
                println!("error in creating sink: {}", e);
            }
        }
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

impl Melody for SwarBlock {
    fn play(
        &self,
        dev: &AudioDevice,
        vol: f32,
        beat_src: Option<Repeat<TakeDuration<Decoder<io::BufReader<fs::File>>>>>,
        _mix: bool,
        n: i8,
    ) {
        for _ in 0..n {
            let mut prev_sw_bt: f32 = 1.0;
            for bt in &self.0 {
                if (prev_sw_bt < 1.0) && (bt.beat_cnt > 1.0) {
                    print!(" {}", bt);
                } else {
                    print!("{}", bt);
                }
                utils::io_flush();
                bt.play(&dev, vol,beat_src.clone(), false, 1);
                prev_sw_bt = bt.beat_cnt;
            }
        }
    }
}
