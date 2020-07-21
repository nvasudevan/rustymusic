extern crate getopts;

use std::env;

use rodio::default_output_device;
use rustymusic::raagas::opts;


fn main() {
    println!("playing hindustani raag's ...");
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();

    let mut opts = getopts::Options::new();
    opts.optopt("z", "rand", "how many notes", "RAND");
    opts.optopt("r", "raag", "which raag to play", "RAAG");
    opts.optopt("f", "play", "play sars from file", "FILE");
    opts.optflag("h", "help", "usage");

    let raag = opts::parse_opts(&opts, args);
    match raag {
        Ok(r) => { r.play(&dev)},
        Err(e) => { opts::print_usage(&e.to_string(), &opts)}
    }
}