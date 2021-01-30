use crate::raagas::raag::raag::Raag;
use crate::raagas::swars::Swar;
use crate::raagas::swarblock::SwarBlock;

pub mod utils;
pub mod constants;
pub mod swarmaalika;
pub mod swars;
pub mod sound;
pub mod taal;
pub mod raag;
pub(crate) mod swarblock;

pub enum Melody {
    SwarBlock(SwarBlock),
    Raag(Raag),
}

pub struct SwarContext {
    swars: Vec<Swar>
}

impl SwarContext {
    pub fn new(swars: Vec<Swar>) -> Self {
        SwarContext {
            swars
        }
    }
}

pub(crate) trait PureRandomiser {
    fn randomise(&self, n_swars: usize) -> Result<Vec<Swar>, String>;
}

pub(crate) trait SimpleRandomiser {
    // mutate Self n times
    fn randomise(&self) -> Option<SwarBlock>;
}

pub(crate) trait Mutate {
    fn mutate(&self,
              i: usize,
              mut_type: SwarBlockMutationType,
              from: Option<Vec<Swar>>) -> Option<SwarBlock>;

    fn mutate_swar(&self, i: usize, from: Option<Vec<Swar>>) -> Option<SwarBlock>;

    fn mutate_swar_duration(&self, i: usize) -> Option<Swar>;
}

pub enum SwarBlockMutationType {
    by_swar,
    by_duration,
}

// pub enum SwarMutationType {
//     simple,
//     increment_beat,
//     decrement_beat,
//     share_beat,
//     kan_swar
// }

pub(crate) trait MutationOperators {
   fn operators(&self) -> Vec<&str>;
}