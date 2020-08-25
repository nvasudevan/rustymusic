use crate::raagas::elements::elements::{AudioDevice, Melody, Swar, Pitch};
use crate::raagas::elements::swarblock::SwarBlock;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub line_a: Option<Vec<SwarBlock>>,
    pub line_b: Option<Vec<SwarBlock>>,
    pub line_c: Option<Vec<SwarBlock>>,
}

impl Sthayi {
    pub fn new(
        line_a: Option<Vec<SwarBlock>>,
        line_b: Option<Vec<SwarBlock>>,
        line_c: Option<Vec<SwarBlock>>,
    ) -> Self {
        Sthayi {
            line_a,
            line_b,
            line_c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub line_c: Option<Vec<SwarBlock>>,
    pub line_d: Option<Vec<SwarBlock>>,
    pub line_e: Option<Vec<SwarBlock>>,
}

impl Antara {
    pub fn new(
        line_c: Option<Vec<SwarBlock>>,
        line_d: Option<Vec<SwarBlock>>,
        line_e: Option<Vec<SwarBlock>>,
    ) -> Self {
        Antara {
            line_c,
            line_d,
            line_e,
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
        let play = |line: &Option<Vec<SwarBlock>>| match &line {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), false, 1);
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
        play(&Some(vec![sam_blk]));

        play(&self.mukra);

        play(&self.sthayi.line_a);
        println!();
        play(&self.sthayi.line_a);
        println!();

        play(&self.sthayi.line_b);
        println!();
        play(&self.sthayi.line_b);
        println!();

        play(&self.sthayi.line_c);
        println!();
        play(&self.sthayi.line_c);
        println!();

        play(&self.sthayi.line_a);
        println!();

        play(&self.antara.line_c);
        println!();
        play(&self.antara.line_c);
        println!();

        play(&self.antara.line_d);
        println!();
        play(&self.antara.line_d);
        println!();

        play(&self.sthayi.line_a);
        println!();

        play(&self.tihayi);
        play(&self.tihayi);
        play(&self.tihayi);
        println!();
    }
}
