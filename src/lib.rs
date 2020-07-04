#[macro_use]
extern crate lazy_static;

pub mod raagas;
mod test;

use crate::raagas::swars;
use std::collections::HashMap;
use std::fs::File;

lazy_static! {
    pub static ref SWARS: HashMap<&'static str, swars::Hertz> = swars::initialise_swars();
    pub static ref BEEP: File = File::open("./samples/beep.wav").unwrap();
}

