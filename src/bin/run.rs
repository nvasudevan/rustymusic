extern crate getopts;

use std::env;

use rodio::default_output_device;
use rustymusic::raagas::opts;


fn main() {
    println!("\nA music application to hindustani compositions using Rust\n");
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();

    let opts = opts::build_opts();
    let raag = opts::parse_opts(&opts, args);
    match raag {
        Ok(r) => { r.play(&dev)},
        Err(e) => { opts::print_usage(&e.to_string(), &opts)}
    }
}