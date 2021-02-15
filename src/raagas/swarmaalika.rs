use std::collections::HashMap;

use crate::raagas::sound::{AudioDevice};

use crate::raagas::swarblock::SwarBlocks;

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lines: HashMap<String, SwarBlocks>,
}

impl Sthayi {
    pub fn new(lines: HashMap<String, SwarBlocks>) -> Self {
        Sthayi { lines }
    }

    pub fn play_line(&self, line: &str, no_times: usize, dev: &AudioDevice) {
        let line_blks = self.lines.get(line);
        if let Some(blks) = line_blks {
            for _ in 0..no_times {
                blks.play(&dev);
            }
        }
    }

    pub fn play(&self, dev: &AudioDevice) {
        self.play_line("lineA", 2, &dev);
        self.play_line("lineB", 2, &dev);
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

    pub fn play_line(&self, line: &str, no_times: usize, dev: &AudioDevice) {
        let line_blks = self.lines.get(line);
        if let Some(blks) = line_blks {
            for _ in 0..no_times {
                blks.play(&dev);
            }
        }
    }

    pub fn play(&self, dev: &AudioDevice) {
        self.play_line("lineC", 2, &dev);
        self.play_line("lineD", 2, &dev);
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

    pub fn play(&self, dev: &AudioDevice) {
        self.sthayi.play(&dev);
        self.sthayi.play_line("lineA", 1, &dev);
        self.antara.play(&dev);
        self.sthayi.play_line("lineA", 1, &dev);
    }
}
