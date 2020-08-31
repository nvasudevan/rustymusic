use getopts::Options;

use crate::raagas::elements::elements::{Melody, Pitch, Swar};
use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::random::randomiser;
use crate::raagas::{raag, utils};
use std::error::Error;

pub fn build_opts() -> Options {
    let mut opts = getopts::Options::new();
    opts.optopt("v", "vol", "set volume", "1.0 (default)");
    opts.optopt("z", "rand", "no of random swars to play", "<-z 5>");
    opts.optopt("r", "raag", "raag to play", "-r <durga|bhupali>");
    opts.optopt("f", "play", "play swars from file", "<file>");
    opts.optflag("h", "help", "usage");

    opts
}

pub fn print_usage(msg: &str, opts: &Options) {
    println!("Usage: {}", opts.usage(msg));
}

pub fn parse_opts<'a>(
    opts: &Options,
    args: Vec<String>,
) -> Result<Box<dyn Melody>, Box<dyn Error>> {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("Nothing matched: {}", e.to_string());
        }
    };

    // play N random notes
    match matches.opt_str("z") {
        Some(n) => {
            let mut swars = randomiser(n.parse::<usize>().unwrap());
            swars.insert(0,Swar::new(Pitch::new("S".to_string()), 3.0));
            let swarblk = SwarBlock(swars);
            return Ok(Box::new(swarblk));
        }
        _ => {}
    }

    // playing swars from the file, although multiple lines, we see it as one block
    match matches.opt_str("f") {
        Some(fp) => {
            println!("Playing swars from the file {}", fp);
            let lines = utils::lines_from_file(fp);
            let mut swars: Vec<Swar> = vec![];

            for l in lines {
                let mut _swars = raag::to_swars(&l);
                swars.append(&mut _swars);
            }

            let swarblk = SwarBlock(swars);
            return Ok(Box::new(swarblk));
        }
        _ => {}
    }

    // playing the raag given
    match matches.opt_str("r") {
        // change case to lower
        Some(r) => {
            let _r = r.to_lowercase();
            println!("playing raag: {}", _r);
            match _r.as_ref() {
                "bhupali" | "durga" | "yaman" | "hamsadhwani" | "yeri_aali" => {
                    let raag = raag::raag(_r).unwrap();
                    Ok(Box::new(raag))
                }
                _ => {
                    Err(format!("Raag {} is unsupported at present, pick one of the supported ragas: bhupali, durga, yaman or hamsadhwani", r).into())
                }
            }
        }
        _ => Err("A valid option was not passed!".into()),
    }
}
