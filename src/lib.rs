#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use crate::raagas::elements::elements::{Hertz, initialise_swars};

pub mod raagas;
mod test;
pub mod opts;

lazy_static! {
    pub static ref SWARS: HashMap<&'static str, Hertz> = initialise_swars();
}
