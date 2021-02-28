use std::error::Error;

use getopts::{Options, Matches};

use crate::raagas::constants::RAAGAS;
use crate::raagas::{raag, Melody, SimpleRandomiser};
use crate::raagas::utils;
use crate::raagas::swarblock;
use crate::raagas::raag::raag::Raag;
use crate::raagas::swarbeat::SwarBeat;

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
    opts.optopt("c", "composition", "play composition", "composition name");
    opts.optopt("f", "play", "play swars from file", "<file>");
    opts.optflag("h", "help", "usage");

    opts
}

fn build_raag(raag: &str, composition: &str) -> Result<Raag, Box<dyn Error>> {
    // is raag supported?
    if !RAAGAS.contains(&raag) {
        let raagas_list = RAAGAS.join(",");
        return Err(
            format!("Raag {} is unsupported, raagas allowed: {}", raag, raagas_list).into(),
        );
    }

    let raag = raag::load::load_yaml(raag, composition).unwrap();
    return Ok(raag);
}

fn parse_composition(matches: &Matches) -> Option<String> {
    if let Some(composition) = matches.opt_str("c") {
        return Some(composition.to_lowercase());
    }

    None
}

fn parse_raag(matches: &Matches) -> Option<String> {
    if let Some(raag) = matches.opt_str("r") {
        return Some(raag.to_lowercase());
    }

    None
}

pub fn parse(
    opts: &Options,
    args: Vec<String>,
) -> Result<Melody, Box<dyn Error>> {
    let matches = opts.parse(&args[1..])?;
    if let Some(r) = parse_raag(&matches) {
        if let Some(c) = parse_composition(&matches) {
            let raag = build_raag(r.as_str(), c.as_str())?;
            // check if play random swars flag is set
            if let Some(_) = matches.opt_str("z") {
                let random_blk = raag.pakad().as_ref().unwrap();
                let swarblk = raag.randomise(random_blk);
                return Ok(Melody::SwarBlocks(swarblk));
            }
            return Ok(Melody::Raag(raag));
        }
    } else {
        // we can play random swars from sargam
    }


    // playing swars from the file
    if let Some(fp) = matches.opt_str("f") {
        println!("Playing swars from the file {}", fp);
        let lines = utils::lines_from_file(fp);
        let mut swarbeats: Vec<SwarBeat> = vec![];

        for l in lines {
            let mut _swars = raag::load::to_swarbeats(&l);
            swarbeats.append(&mut _swars);
        }

        let swarblk = swarblock::SwarBlock(swarbeats);
        return Ok(Melody::SwarBlock(swarblk));
    }

    return Err("Invalid options passed".into());
}
