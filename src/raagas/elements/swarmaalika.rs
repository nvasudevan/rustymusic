use crate::raagas::elements::elements::{AudioDevice, Melody, Swar, Pitch};
use crate::raagas::elements::swarblock::SwarBlock;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lines: HashMap<String, Vec<SwarBlock>>
}

impl Sthayi {
    pub fn new(lines: HashMap<String, Vec<SwarBlock>>) -> Self {
        Sthayi {
            lines
        }
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub lines: HashMap<String, Vec<SwarBlock>>
}

impl Antara {
    pub fn new(lines: HashMap<String, Vec<SwarBlock>>) -> Self {
        Antara {
            lines
        }
    }
}

#[derive(Debug, Clone)]
pub struct Swarmaalika {
    pub mukra: Option<Vec<SwarBlock>>,
    pub sthayi: Sthayi,
    pub antara: Antara,
    pub tihayi: Option<Vec<SwarBlock>>,
    pub sam: usize,
}

impl Swarmaalika {
    pub fn new(
        mukra: Option<Vec<SwarBlock>>,
        sthayi: Sthayi,
        antara: Antara,
        tihayi: Option<Vec<SwarBlock>>,
        sam: Option<usize>
    ) -> Self {
        let mut _sam = match sam {
            Some(n) => {
                n
            },
            _ => {
                1
            }
        };
        Swarmaalika {
            mukra,
            sthayi,
            antara,
            tihayi,
            sam: _sam
        }
    }
}

impl Melody for Swarmaalika {
    // TODO: should the beat_src be a reference (&beat_src)?
    // [mukra] <sthayi> A <antara> A <tihayi> X 3
    // [mukra] <A A B B [C]> A <C C D D E E] A <tihayi> X 3
    fn play(
        &self,
        dev: &AudioDevice,
        beat_src: Option<Repeat<TakeDuration<Decoder<BufReader<File>>>>>,
        mix: bool,
        _n: i8,
    ) {
        // play: sthayi, line A of sthayi, antara, line A of sthayi, tihayi
        println!("\nPlaying swarmaalika");
        let play = |line: &Option<Vec<SwarBlock>>, cnt: usize| match &line {
            Some(line) => {
                for blk in line {
                    for i in 0..cnt {
                        blk.play(&dev, beat_src.clone(), false, 1);
                        println!();
                    }
                }
            }
            _ => {}
        };

        // play taal until sam - count(mukra)
        let _sam = match &self.mukra {
            Some(mukra) => {
                let mut _n = 0;
                for blk in mukra {
                    _n = _n + blk.0.len();
                }
                self.sam - _n
            },
            _ => {
                self.sam
            }
        };
        let sam_blk = SwarBlock::from_iter((0.._sam).into_iter());
        play(&Some(vec![sam_blk]), 1);

        play(&self.mukra, 1);

        let sthayi_tags: Vec<&str> = vec!["lineA", "lineB", "lineC"];
        for t in sthayi_tags {
            let line = self.sthayi.lines.get(t);
            match line {
                Some(l) => {
                    let _line = l.to_owned();
                    play(&Some(_line), 2);
                },
                _ => {}
            }
        }

        // antara
        let antara_tags: Vec<&str> = vec!["lineC", "lineD", "lineE"];
        for t in antara_tags {
            let line = self.antara.lines.get(t);
            match line {
                Some(l) => {
                    let _line = l.to_owned();
                    play(&Some(_line), 2);
                },
                _ => {}
            }
        }

        play(&self.tihayi, 3);
        println!();
    }
}
