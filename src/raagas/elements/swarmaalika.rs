use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::elements::elements::{Melody, AudioDevice};
use rodio::source::{Repeat, TakeDuration};
use rodio::decoder::Decoder;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lineA: Option<Vec<SwarBlock>>,
    pub lineB: Option<Vec<SwarBlock>>,
    pub lineC: Option<Vec<SwarBlock>>,
}

impl Sthayi {
    pub fn new(lineA: Option<Vec<SwarBlock>>, lineB: Option<Vec<SwarBlock>>, lineC: Option<Vec<SwarBlock>>) -> Self {
        Sthayi {
            lineA,
            lineB,
            lineC
        }
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub lineC: Option<Vec<SwarBlock>>,
    pub lineD: Option<Vec<SwarBlock>>,
    pub lineE: Option<Vec<SwarBlock>>,
}

impl Antara {
    pub fn new(lineC: Option<Vec<SwarBlock>>, lineD: Option<Vec<SwarBlock>>, lineE: Option<Vec<SwarBlock>>) -> Self {
        Antara  {
            lineC,
            lineD,
            lineE
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
    pub fn new(mukra: Option<Vec<SwarBlock>>,
               sthayi: Sthayi,
               antara: Antara,
               tihayi: Option<Vec<SwarBlock>>) -> Self {
        Swarmaalika { mukra, sthayi, antara, tihayi }
    }
}

impl Melody for Swarmaalika {
    // TODO: should the beat_src be a reference (&beat_src)?
    // [mukra] <sthayi> A <antara> A <tihayi> X 3
    // [mukra] <A A B B [C]> A <C C D D E E] A <tihayi> X 3
    fn play(&self, dev: &AudioDevice, beat_src: Repeat<TakeDuration<Decoder<BufReader<File>>>>, n: i8) {
        // play: sthayi, line A of sthayi, antara, line A of sthayi, tihayi
        println!("\nPlaying swarmaalika");
        let play = |line: &Option<Vec<SwarBlock>>| {
            match &line {
                Some(line) => {
                    for blk in line {
                        blk.play(&dev, beat_src.clone(), 1);
                    };
                },
                _ => {}
            }
        };

        play(&self.mukra);

        play(&self.sthayi.lineA);
        println!();
        play(&self.sthayi.lineA);
        println!();

        play(&self.sthayi.lineB);
        println!();
        play(&self.sthayi.lineB);
        println!();

        play(&self.sthayi.lineC);
        println!();
        play(&self.sthayi.lineC);
        println!();

        play(&self.sthayi.lineA);
        println!();

        play(&self.antara.lineC);
        println!();
        play(&self.antara.lineC);
        println!();

        play(&self.antara.lineD);
        println!();
        play(&self.antara.lineD);
        println!();

        play(&self.sthayi.lineA);
        println!();

        play(&self.tihayi);
        play(&self.tihayi);
        play(&self.tihayi);
        println!();

    }
}

