extern crate getopts;

use std::env;

use rustymusic::opts;
use rustymusic::raagas::sound::AudioDevice;
use rustymusic::raagas::constants;

use rustymusic::raagas::Melody;

fn main() {
    match rodio::OutputStream::try_default() {
        Ok(out) => {
            let (_, stream_handle) = out;
            let opts = opts::my_opts();
            match opts::parse(&opts, env::args().collect()) {
                Ok(melody) => {
                    let audio_dev = AudioDevice::new(stream_handle);
                    if let Melody::Raag(raag) = &melody {
                        raag.play(&audio_dev, constants::VOL, None, false, 1)
                    }

                    if let Melody::SwarBlock(blk) = &melody {
                        // blk.play_rt(&audio_dev, constants::VOL)
                    }
                },
                Err(e) => opts::print_usage(&e.to_string(), &opts),
            }
        },
        Err(e) => {
            println!("Error getting an output device: {}", e);
        }
    }
}
