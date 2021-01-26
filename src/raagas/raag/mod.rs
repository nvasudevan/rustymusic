use crate::raagas::raag::raag::Raag;
use crate::raagas::swars::{SwarBlock, Swar};

pub mod load;
pub mod raag;
pub mod random;

pub enum Melody {
    SwarBlock(SwarBlock),
    Raag(Raag),
}

pub(crate) trait PureRandomiser {
    fn randomise(&self, n_swars: usize) -> Result<Vec<Swar>, String>;
}

pub(crate) trait SimpleRandomiser {
    // mutate Self n times
    fn randomise(&self) -> SwarBlock;
}
