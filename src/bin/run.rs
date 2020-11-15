extern crate getopts;

use std::env;

use rodio::default_output_device;
use rustymusic::raagas::elements::elements;
use rustymusic::raagas::opts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();
    pub const VOL: f32 = 0.5;

    let opts = opts::build_opts();
    let audio_dev = elements::AudioDevice::new(dev, VOL);

    let comp = opts::parse_opts(&opts, args);
    match comp {
        Ok(c) => c.play(&audio_dev, None, false, 1),
        Err(e) => opts::print_usage(&e.to_string(), &opts),
    }
}