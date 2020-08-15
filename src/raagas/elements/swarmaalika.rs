use crate::raagas::elements::elements::{AudioDevice, Melody};
use crate::raagas::elements::swarblock::SwarBlock;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::fs::File;
use std::io::BufReader;

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
}

impl Swarmaalika {
    pub fn new(
        mukra: Option<Vec<SwarBlock>>,
        sthayi: Sthayi,
        antara: Antara,
        tihayi: Option<Vec<SwarBlock>>,
    ) -> Self {
        Swarmaalika {
            mukra,
            sthayi,
            antara,
            tihayi,
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
        beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>,
        _n: i8,
    ) {
        // play: sthayi, line A of sthayi, antara, line A of sthayi, tihayi
        println!("\nPlaying swarmaalika");
        let play = |line: &Option<Vec<SwarBlock>>| match &line {
            Some(line) => {
                for blk in line {
                    blk.play(&dev, beat_src.clone(), 1);
                }
            }
            _ => {}
        };

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
