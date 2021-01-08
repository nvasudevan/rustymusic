extern crate getopts;

use std::env;

use rodio::default_output_device;

use rustymusic::raagas::elements::elements;
use rustymusic::opts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();
    pub const VOL: f32 = 0.5;
    let audio_dev = elements::AudioDevice::new(dev, VOL);

    let opts = opts::my_opts();

    match opts::parse(&opts, args) {
        Ok(melody) => {
            melody.play(&audio_dev, None, false, 1)
        },
        Err(e) => opts::print_usage(&e.to_string(), &opts),
    }
}
