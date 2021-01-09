extern crate getopts;

use std::env;

use rustymusic::opts;
use rustymusic::raagas::physics::AudioDevice;
use rustymusic::raagas::constants;
use rodio::Sink;

fn main() {
    match rodio::OutputStream::try_default() {
        Ok(out) => {
            let (_, stream_handle) = out;
            let opts = opts::my_opts();
            match opts::parse(&opts, env::args().collect()) {
                Ok(melody) => {
                    let audio_dev = AudioDevice::new(stream_handle);
                    melody.play(&audio_dev, constants::VOL, None, false, 1)
                },
                Err(e) => opts::print_usage(&e.to_string(), &opts),
            }
        },
        Err(e) => {
            println!("Error getting an output device: {}", e);
        }
    }
}
