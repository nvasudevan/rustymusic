use crate::raagas::raag::raag::Raag;
use crate::raagas::swars::Swar;
use crate::raagas::swarblock::{SwarBlock, SwarInSwarBlock};
use crate::raagas::swarblocks::SwarBlocks;

pub mod utils;
pub mod constants;
pub mod swarmaalika;
pub mod swars;
pub mod sound;
pub mod taal;
pub mod raag;
pub(crate) mod swarblock;
pub(crate) mod swarbeat;
mod swarblocks;
mod aroha_avroha;

pub enum Melody {
    SwarBlocks(SwarBlocks),
    SwarBlock(SwarBlock),
    Raag(Raag),
}

pub(crate) trait PureRandomiser {
    fn randomise(&self, n_swars: usize) -> Result<Vec<Swar>, String>;
}

pub(crate) trait SimpleRandomiser {
    // mutate Self n times
    fn randomise(&self, src_blks: &SwarBlocks) -> SwarBlocks;
}

pub(crate) trait Mutate {
    fn mutate(&self,
              index: &SwarInSwarBlock,
              from: Vec<&Swar>) -> Self;

    fn mutate_swar(&self, index: &SwarInSwarBlock, from: Vec<&Swar>) -> Self;

    fn mutate_swar_duration(&self, i: usize) -> Option<Swar>;
}

pub enum SwarBlockMutationType {
    BySwar,
    ByDuration,
}

pub(crate) trait MutationOperators {
    fn operators(&self) -> Vec<&str>;
    fn random_mutation_operator(&self) -> String;
}