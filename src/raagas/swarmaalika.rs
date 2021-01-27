use std::collections::HashMap;

use crate::raagas::sound::{AudioDevice, TimedSink};
use crate::raagas::swars::BeatSrc;

use rodio::PlayError;
use crate::raagas::swarblock::SwarBlocks;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lines: HashMap<String, SwarBlocks>,
}

impl Sthayi {
    pub fn new(lines: HashMap<String, SwarBlocks>) -> Self {
        Sthayi { lines }
    }

    pub fn build_sink(&self,
                       beat_src: &Option<BeatSrc>,
                       dev: &AudioDevice,
                       vol: f32) -> Result<Vec<TimedSink>, PlayError> {

        let mut sinks: Vec<TimedSink> = Vec::new();
        for (key, line) in self.lines.iter() {
            match key.as_str() {
                "lineA" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                "lineB" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                "lineC" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                _ => {}
            }
        }

        match self.lines.get("lineA") {
            Some(line_a) => {
                let mut sink = line_a.build_sink(&beat_src, &dev, vol)?;
                sinks.append(&mut sink);
            },
            _ => {}
        }

        Ok(sinks)
    }
}

#[derive(Debug, Clone)]
pub struct Antara {
    pub lines: HashMap<String, SwarBlocks>,
}

impl Antara {
    pub fn new(lines: HashMap<String, SwarBlocks>) -> Self {
        Antara { lines }
    }

    pub fn build_sink(&self,
                       beat_src: &Option<BeatSrc>,
                       dev: &AudioDevice,
                       vol: f32) -> Result<Vec<TimedSink>, PlayError> {

        let mut sinks: Vec<TimedSink> = Vec::new();
        for (key, line) in self.lines.iter() {
            match key.as_str() {
                "lineC" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                "lineD" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                "lineE" => {
                    let mut sink = line.build_sink(&beat_src, &dev, vol)?;
                    sinks.append(&mut sink);
                },
                _ => {}
            }
        }

        Ok(sinks)
    }
}

#[derive(Debug, Clone)]
pub struct Swarmaalika {
    pub mukra: Option<SwarBlocks>,
    pub sthayi: Sthayi,
    pub antara: Antara,
    pub tihayi: Option<SwarBlocks>,
    pub sam: usize,
}

impl Swarmaalika {
    pub fn new(
        mukra: Option<SwarBlocks>,
        sthayi: Sthayi,
        antara: Antara,
        tihayi: Option<SwarBlocks>,
        sam: Option<usize>,
    ) -> Self {
        let mut _sam = match sam {
            Some(n) => n,
            _ => 1,
        };

        Swarmaalika {
            mukra,
            sthayi,
            antara,
            tihayi,
            sam: _sam,
        }
    }

    pub fn build_sink(&self,
                       beat_src: &Option<BeatSrc>,
                       dev: &AudioDevice,
                       vol: f32) -> Result<Vec<TimedSink>, PlayError>
    {
        let mut sinks: Vec<TimedSink> = Vec::new();
        let mut sthayi = self.sthayi.build_sink(&beat_src, &dev, vol)?;
        sinks.append(&mut sthayi);
        let mut anthara = self.antara.build_sink(&beat_src, &dev, vol)?;
        sinks.append(&mut anthara);
        Ok(sinks)
    }
}
