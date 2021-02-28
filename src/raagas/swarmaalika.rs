use std::collections::HashMap;

use crate::raagas::sound::{AudioDevice};

use crate::raagas::swarblock::SwarBlocks;

#[derive(Debug, Clone)]
pub struct Swarmaalika {
    pub mukra: Option<SwarBlocks>,
    pub sthayi: Sthayi,
    pub antara: Antara,
    pub tihayi: Option<SwarBlocks>,
    sam: usize,
}

#[derive(Debug, Clone)]
pub struct Sthayi {
    pub lines: HashMap<String, SwarBlocks>,
}

impl Sthayi {
    pub fn new(lines: HashMap<String, SwarBlocks>) -> Self {
        Sthayi { lines }
    }

    pub fn play_line(&self, line: &str, no_times: usize, dev: &AudioDevice) {
        println!("line: {}", line);
        let line_blks = self.lines.get(line);
        if let Some(blks) = line_blks {
            println!("  {}", blks);
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
        println!("line: {}", line);
        let line_blks = self.lines.get(line);
        if let Some(blks) = line_blks {
            println!("  {}", blks);
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

impl Swarmaalika {
    pub fn new(
        mukra: Option<SwarBlocks>,
        sthayi: Sthayi,
        antara: Antara,
        tihayi: Option<SwarBlocks>,
        sam: Option<usize>,
    ) -> Self {
        let my_sam = match sam {
            Some(n) => n,
            _ => 1,
        };

        Swarmaalika {
            mukra,
            sthayi,
            antara,
            tihayi,
            sam: my_sam,
        }
    }

    pub fn sam(&self) -> usize {
        self.sam
    }

    pub fn play(&self, dev: &AudioDevice) {
        self.sthayi.play(&dev);
        self.sthayi.play_line("lineA", 1, &dev);
        self.antara.play(&dev);
        self.sthayi.play_line("lineA", 1, &dev);
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;

    /// test if we can retrieve a line from Sthayi and match a swarbeat
    #[test]
    fn test_line_in_swarmaalika() {
        let raag = "malkauns";
        let composition = "comp1";
        let line_a = "lineA";

        let raag = load::load_yaml(raag, composition).unwrap();
        let blks = raag.swarmaalika().sthayi.lines.get(line_a);
        assert!(blks.is_some());
    }

    /// test if we can retrieve a line from Sthayi and match a swarbeat
    #[test]
    fn test_swarbeat_from_line_in_swarmaalika() {
        let raag = "malkauns";
        let composition = "comp1";
        let line_a = "lineA";
        let sw_bt_index: usize = 6;
        let sw_bt_expected = "M:d:M:g";

        let raag = load::load_yaml(raag, composition).unwrap();
        let blks = raag.swarmaalika().sthayi.lines.get(line_a);
        assert_eq!(blks.unwrap().swarbeats().get(sw_bt_index).unwrap().to_string(), sw_bt_expected);
    }

}