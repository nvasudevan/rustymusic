use std::error::Error;

use getopts::Options;

use crate::raagas::swars::{Melody, Swar, SwarBlock};
use crate::raagas::constants::RAAGAS;
use crate::raagas::random::randomiser;
use crate::raagas::raag;
use crate::raagas::physics::Pitch;

pub fn print_usage(msg: &str, opts: &Options) {
    println!("Usage: {}", opts.usage(msg));
}

pub fn my_opts() -> Options {
    let mut opts = getopts::Options::new();
    opts.optopt("v", "vol", "set volume", "1.0 (default)");
    opts.optopt(
        "z",
        "rand",
        "no of random swars to play for a raag",
        "<-z 5>",
    );
    let supported_raagas = RAAGAS.join(",");
    opts.optopt("r", "raag", "raag to play",
                &format!("-r {}", supported_raagas));
    opts.optopt("f", "play", "play swars from file", "<file>");
    opts.optflag("h", "help", "usage");

    opts
}

pub fn parse(
    opts: &Options,
    args: Vec<String>,
) -> Result<Box<dyn Melody>, Box<dyn Error>> {
    let matches = opts.parse(&args[1..])?;
    match matches.opt_str("r") {
        Some(r) => {
            let _r = r.to_lowercase();
            // is raag supported?
            if !RAAGAS.contains(&_r.as_str()) {
                let _raagas = RAAGAS.join(",");
                return Err(
                    format!("Raag {} is unsupported, raagas allowed: {}", r, _raagas).into(),
                );
            }
            let raag = raag::load::load_yaml(_r).unwrap();

            // check if play random swars flag is set
            if let Some(n) = matches.opt_str("z") {
                // let swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
                let rnd_swars = randomiser(&raag, n.parse::<usize>().unwrap());
                match rnd_swars {
                    Ok(mut _swars) => {
                        _swars.insert(0,Swar::new(Pitch::new("S".to_string()), 3.0));
                        let swarblk = SwarBlock(_swars);
                        return Ok(Box::new(swarblk));
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            } else {
                return Ok(Box::new(raag));
            }
        }
        _ => Err("A valid option was not passed!".into()),
    }

    // playing swars from the file, although multiple lines, we see it as one block
    // match matches.opt_str("f") {
    //     Some(fp) => {
    //         println!("Playing swars from the file {}", fp);
    //         let lines = utils::lines_from_file(fp);
    //         let mut swars: Vec<Swar> = vec![];
    //
    //         for l in lines {
    //             let mut _swars = raag::to_swars(&l);
    //             swars.append(&mut _swars);
    //         }
    //
    //         let swarblk = SwarBlock(swars);
    //         return Ok(Box::new(swarblk));
    //     }
    //     _ => {}
    // }
}
