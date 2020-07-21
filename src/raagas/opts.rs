use getopts::Options;

use crate::raagas::elements::{Melody, SwarBlock, Beat, Pitch, BASE_SWAR_INTERVAL};
use std::error::Error;
use crate::raagas::random::randomiser;
use crate::raagas::raag;

pub fn build_opts() -> Options {
    let mut opts = getopts::Options::new();
    opts.optopt("z", "rand", "no of random swars to play", "<-z 5>");
    opts.optopt("r", "raag", "raag to play", "-r <durga|bhupali>");
    opts.optopt("f", "play", "play swars from file", "<file>");
    opts.optflag("h", "help", "usage");

    opts
}

pub fn print_usage(msg: &str, opts: &Options) {
    println!("Usage: {}", opts.usage(msg));
}

pub fn parse_opts<'a>(opts: &Options, args: Vec<String>) -> Result<Box<dyn Melody>, Box<dyn Error>> {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m},
        Err(e) => {
            panic!("Nothing matched: {}", e.to_string());
        }
    };

    // play N random notes
    match matches.opt_str("z") {
        Some(n) => {
            let beats = randomiser(n.parse::<u32>().unwrap());
            let swarblk = SwarBlock(beats);
            return Ok(Box::new(swarblk));
        }
        _ => {}
    }

    // playing swars from the file
    match matches.opt_str("f") {
        Some(fp) => {
            println!("Playing swars from the file {}", fp);
            let s = std::fs::read_to_string(fp).unwrap();
            let _s = s.replace("\n", "");
            let swars: Vec<String> = _s.split(" ").map(|x| x.to_ascii_uppercase()).collect();
            println!("swars: {:?}", swars);
            let mut beats: Vec<Beat> = vec![];
            for swr in swars {
                if swr.eq("-") {
                    let prev = beats.pop().unwrap();
                    let long = prev.long + 1;

                    beats.push(Beat {swar: prev.swar, long: long});

                } else {
                    beats.push(
                        Beat { swar: Some(Pitch::new(swr)),
                            long: BASE_SWAR_INTERVAL }
                    );
                }
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
                    let fp = format!("./config/durga");
                    let raag = raag::raag("Durga".to_string(), fp);
                    Ok(Box::new(raag))
                }
                "bhupali" => {
                    let fp = format!("./config/bhupali");
                    let raag = raag::raag("Bhupali".to_string(), fp);
                    Ok(Box::new(raag))
                }
                _ => {
                    Err(format!("Raag {} is unfamiliar, so can't play", r).into())
                }
            }
        }
        _ => {
            Err("A valid option was not passed!".into())
        }
    }
}
