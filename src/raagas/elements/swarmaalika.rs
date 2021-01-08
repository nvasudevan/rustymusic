use crate::raagas::elements::elements::{AudioDevice, Melody, Swar};
use crate::raagas::elements::swarblock::SwarBlock;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lines: HashMap<String, Vec<SwarBlock>>,
}

impl Sthayi {
    pub fn new(lines: HashMap<String, Vec<SwarBlock>>) -> Self {
        Sthayi { lines }
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub lines: HashMap<String, Vec<SwarBlock>>,
}

impl Antara {
    pub fn new(lines: HashMap<String, Vec<SwarBlock>>) -> Self {
        Antara { lines }
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
        sam: Option<usize>,
    ) -> Self {
        let mut _sam = match sam {
            Some(n) => n,
            _ => 1,
        };
        // let mut _tihayi = match tihayi {
        //     Some(n) => {
        //         n
        //     },
        //     _ => {
        //         1
        //     }
        // };
        Swarmaalika {
            mukra,
            sthayi,
            antara,
            tihayi,
            sam: _sam,
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
        _mix: bool,
        _n: i8,
    ) {
        println!("\n=> swarmaalika");
        let play = |line: &Option<Vec<SwarBlock>>, cnt: usize| match &line {
            Some(line) => {
                for blk in line {
                    for _ in 0..cnt {
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
            }
            _ => self.sam,
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
                }
                _ => {}
            }
        }

        // after sthayi play A
        match self.sthayi.lines.get("lineA") {
            Some(l) => {
                let _line = l.to_owned();
                play(&Some(_line), 1);
            }
            _ => {}
        }

        // antara
        let antara_tags: Vec<&str> = vec!["lineC", "lineD", "lineE"];
        for t in antara_tags {
            let line = self.antara.lines.get(t);
            match line {
                Some(l) => {
                    let _line = l.to_owned();
                    play(&Some(_line), 2);
                }
                _ => {}
            }
        }

        // match lineA {
        //     Some(l) => {
        //         let _line = l.to_owned();
        //         play(&Some(_line.clone()), 1);
        //         println!();
        //
        //         // play tihayi
        //         // let mut i = 0;
        //         // let mut tihayi_blk: Vec<Swar> = Vec::new();
        //         // let _line_tihayi = _line.clone();
        //         // for blk in _line_tihayi {
        //         //     for sw in blk.0 {
        //         //         let cnt = &sw.beat_cnt;
        //         //         i += *cnt as usize;
        //         //         tihayi_blk.push(sw);
        //         //         if i >= self.tihayi {
        //         //             break
        //         //         }
        //         //     }
        //         //     if i >= self.tihayi {
        //         //         break
        //         //     }
        //         // }
        //         // println!("tihayi: {:?}", tihayi_blk);
        //         // play(&Some(vec![SwarBlock(tihayi_blk)]), 3);
        //         //
        //         // // finally, now play the remaining swars of line A
        //         // let mut j: usize = 0;
        //         // for blk in _line {
        //         //     for sw in blk.0 {
        //         //         let cnt = &sw.beat_cnt;
        //         //         j += *cnt as usize;
        //         //         if j > self.tihayi {
        //         //             let _sw = sw.to_owned();
        //         //             let _sw_ext = Swar::new(_sw.pitch.unwrap(), 2.0);
        //         //             _sw_ext.play(&dev, None, false, 1);
        //         //         }
        //         //     }
        //         // }
        //     },
        //     _ => {}
        // }

        // play tihayi
        let mut t_cnt: usize = 0;
        for blk in self.tihayi.as_ref().unwrap() {
            t_cnt = t_cnt + blk.no_beats();
        }
        println!("tihayi no of beats: {}", t_cnt);
        play(&self.tihayi, 3);

        // we need to play swars from line A
        // from the beat where tihayi finishes until beat cycle finishes
        match self.sthayi.lines.get("lineA") {
            Some(l) => {
                let _line = l.to_owned();
                let mut j: usize = 0;
                for blk in _line {
                    for sw in blk.0 {
                        let cnt = &sw.beat_cnt;
                        j += *cnt as usize;
                        if j > t_cnt {
                            let _sw = sw.to_owned();
                            let _sw_ext = Swar::new(_sw.pitch.unwrap(), 2.0);
                            _sw_ext.play(&dev, None, false, 1);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
