use std::error::Error;

use getopts::Options;

use crate::raagas::swars::Swar;
use crate::raagas::constants::RAAGAS;
use crate::raagas::{raag, Melody, SimpleRandomiser};
use crate::raagas::utils;
use crate::raagas::swarblock;

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

fn parse_raag(raag: &str) -> Result<Melody, Box<dyn Error>> {
    // is raag supported?
    if !RAAGAS.contains(&raag) {
        let raagas_list = RAAGAS.join(",");
        return Err(
            format!("Raag {} is unsupported, raagas allowed: {}", raag, raagas_list).into(),
        );
    }

    let _raag = raag::load::load_yaml(raag.to_string()).unwrap();
    return Ok(Melody::Raag(_raag));
}

pub fn parse(
    opts: &Options,
    args: Vec<String>,
) -> Result<Melody, Box<dyn Error>> {
    let matches = opts.parse(&args[1..])?;
    // check if play random swars flag is set
    if let Some(_n) = matches.opt_str("z") {
        if let Some(r) = matches.opt_str("r") {
            let melody = parse_raag(r.to_lowercase().as_str())?;
            if let Melody::Raag(raag) = &melody {
                // let swars = raag.randomise_swarblocks(raag.pakad().as_ref().unwrap().0.get(0).unwrap());
                // let choose_from: Vec<Swar> = raag.aroha().as_ref().unwrap().to_swars();
                if let Some(swarblk) = raag.randomise() {
                    // println!("=> swar block: {}", swarblk);
                    return Ok(Melody::SwarBlock(swarblk));
                }
                // let swarblk = SwarBlock(swars.to_swars());
                return Ok(Melody::SwarBlock(raag.pakad().as_ref().unwrap().to_swarblock()));
            }
        } else {
            // we can play random swars from sargam
        }
    }

    if let Some(r) = matches.opt_str("r") {
        return parse_raag(r.to_lowercase().as_str());
    }

    // playing swars from the file
    if let Some(fp) = matches.opt_str("f") {
        println!("Playing swars from the file {}", fp);
        let lines = utils::lines_from_file(fp);
        let mut swars: Vec<Swar> = vec![];

        for l in lines {
            let mut _swars = raag::load::to_swars( & l);
            swars.append( & mut _swars);
        }

        let swarblk = swarblock::SwarBlock(swars);
        return Ok(Melody::SwarBlock(swarblk));
    }

    return Err("Invalid options passed".into());
}
