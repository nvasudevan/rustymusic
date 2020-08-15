#[macro_use]
extern crate lazy_static;

pub mod raagas;
mod test;

use std::collections::HashMap;

use crate::raagas::elements::elements::{initialise_swars, Hertz};

lazy_static! {
    pub static ref SWARS: HashMap<&'static str, Hertz> = initialise_swars();
}
