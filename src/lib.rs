#[macro_use]
extern crate lazy_static;

pub mod raagas;
mod test;

use std::collections::HashMap;
use std::fs::File;

use crate::raagas::elements::{Hertz, initialise_swars};

lazy_static! {
    pub static ref SWARS: HashMap<&'static str, Hertz> = initialise_swars();
    pub static ref BEEP: File = File::open("./samples/beep.wav").unwrap();
}

