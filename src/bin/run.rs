extern crate getopts;

use std::env;
use getopts::Options;
use rand;
use rand::{Rng, random};
use rodio::{Sink, source::SineWave, default_output_device, output_devices, Device};
use rustymusic::raagas::{elements, bhupali, durga, swars};
use rustymusic::raagas::swars::Pitch;
use rustymusic::raagas::elements::{Melody, Raag, SwarBlock, Beat};
use std::error::Error;


fn rand_next() -> Option<u8> {
    let mut rnd_num = rand::thread_rng();
    let n: u8 = rnd_num.gen();

    Some(n)
}

fn print_usage(msg: &str, opts: &Options) {
    println!("Usage: {}", opts.usage(msg));
}

fn parse_opts<'a>(opts: &Options, args: Vec<String>) -> Result<Box<dyn Melody>, Box<dyn Error>> {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m},
        Err(e) => {
            panic!("Nothing matched: {}", e.to_string());
        }
    };

    // playing swars from the file
    match matches.opt_str("f") {
        Some(fp) => {
            println!("Playing swars from the file {}", fp);
            let s = std::fs::read_to_string(fp).unwrap();
            let _s = s.replace("\n", "");
            let swars: Vec<String> = _s.split(" ").map(|x| x.to_string().to_ascii_uppercase()).collect();
            println!("swars: {:?}", swars);
            let mut beats: Vec<Beat> = vec![];
            for swr in swars {
                beats.push(
                    Beat { swar: Some(Pitch::new(swr)),
                                long: swars::BASE_SWAR_INTERVAL }
                );
            }
            let swarblk = SwarBlock(beats);
            return Ok(Box::new(swarblk));
        }
        _ => {}
    }

    // playing the raag given
    match matches.opt_str("r") {
        Some(r) => {
            println!("playing raag: {}", r);
            match r.as_ref() {
                "durga" => {
                    let raag = durga::durga();
                    Ok(Box::new(raag))
                }
                "bhupali" => {
                    let raag = bhupali::bhupali();
                    Ok(Box::new(raag))
                }
                _ => {
                    Err(format!("Raag {} is unfamiliar, so can't play", r).into())
                }
            }
        }
        _ => {
            Err("A valid raag was not passed!".into())
        }
    }
}

fn main() {
    println!("playing hindustani raag's ...");
    let args: Vec<String> = env::args().collect();
    let prg = &args[0];
    let dev = default_output_device().unwrap();

    let mut opts = getopts::Options::new();
    opts.optopt("r", "raag", "which raag to play", "RAAG");
    opts.optopt("f", "play", "play sars from file", "FILE");
    opts.optflag("h", "help", "usage");

    let raag = parse_opts(&opts, args);
    match raag {
        Ok(r) => { r.play(&dev)},
        Err(e) => { print_usage(&e.to_string(), &opts)}
    }

    // play_sargam(&dev);
}