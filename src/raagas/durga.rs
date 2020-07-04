use crate::raagas::swars;
use crate::raagas::elements::{Raag, Beat, SwarBlock};
use rodio::Device;
use crate::raagas::swars::Pitch;

pub fn durga<'a>() -> Raag<'a> {
    let mut _aroha: Vec<Beat> = vec![];
    _aroha.push(Beat { swar: None, long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::default()), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("RE")), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("MA")), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("PA")), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("DHA")), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("SA+")), long: 2*swars::BASE_SWAR_INTERVAL });

    let mut _avroha: Vec<Beat> = vec![];
    _avroha.push(Beat { swar: Some(Pitch::new("SA+")), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("DHA")), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("PA")), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("MA")), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("RE")), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::default()), long: 2*swars::BASE_SWAR_INTERVAL });

    let mut _pakad: Vec<SwarBlock> = vec![];
    let mut _blk1: Vec<Beat> = vec![];
    _blk1.push(Beat { swar: Some(Pitch::new("MA")), long: swars::BASE_SWAR_INTERVAL });
    _blk1.push(Beat { swar: Some(Pitch::new("PA")), long: swars::BASE_SWAR_INTERVAL });
    _blk1.push(Beat { swar: Some(Pitch::new("DHA")), long: 2*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk1));

    let mut _blk2: Vec<Beat> = vec![];
    _blk2.push(Beat { swar: Some(Pitch::new("MA")), long: swars::BASE_SWAR_INTERVAL });
    _blk2.push(Beat { swar: Some(Pitch::new("RE")), long: 2*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk2));

    let mut _blk3: Vec<Beat> = vec![];
    _blk3.push(Beat { swar: Some(Pitch::new("-DHA")), long: swars::BASE_SWAR_INTERVAL });
    _blk3.push(Beat { swar: Some(Pitch::new("-DHA")), long: swars::BASE_SWAR_INTERVAL });
    _blk3.push(Beat { swar: Some(Pitch::default()), long: 2*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk3));

    Raag::new(String::from("Durga"), _aroha,  _avroha, _pakad)
}
