extern crate getopts;

use std::env;

use rodio::default_output_device;
use rustymusic::raagas::opts;
use rustymusic::raagas::elements;


fn main() {
    println!("\nProgram to render Hindustani compositions written in Rust\n");
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();

    let opts = opts::build_opts();
    let raag = opts::parse_opts(&opts, args);
    pub const VOL: f32 = 2.0;
    let audio_dev = elements::AudioDevice::new(dev, VOL);
    match raag {
        Ok(r) => { r.play(&audio_dev)},
        Err(e) => { opts::print_usage(&e.to_string(), &opts)}
    }
}