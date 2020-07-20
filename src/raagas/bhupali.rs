use crate::raagas::swars;
use crate::raagas::elements::{Raag, Beat, SwarBlock};
use rodio::Device;
use crate::raagas::swars::Pitch;

pub fn bhupali() -> Raag {
    let mut _aroha: Vec<Beat> = vec![];
    _aroha.push(Beat { swar: Some(Pitch::default()), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("RE".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("PA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("DHA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _aroha.push(Beat { swar: Some(Pitch::new("SA+".to_string())), long: swars::BASE_SWAR_INTERVAL });

    let mut _avroha: Vec<Beat> = vec![];
    _avroha.push(Beat { swar: Some(Pitch::new("SA+".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("DHA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("PA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::new("RE".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _avroha.push(Beat { swar: Some(Pitch::default()), long: swars::BASE_SWAR_INTERVAL });

    let mut _pakad: Vec<SwarBlock> = vec![];
    let mut _blk1: Vec<Beat> = vec![];
    _blk1.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk1.push(Beat { swar: Some(Pitch::new("RE".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk1.push(Beat { swar: Some(Pitch::default()), long: swars::BASE_SWAR_INTERVAL });
    _blk1.push(Beat { swar: Some(Pitch::new("-DHA".to_string())), long: 3*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk1));

    let mut _blk2: Vec<Beat> = vec![];
    _blk2.push(Beat { swar: Some(Pitch::default()), long: swars::BASE_SWAR_INTERVAL });
    _blk2.push(Beat { swar: Some(Pitch::new("RE".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk2.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: 3*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk2));

    let mut _blk3: Vec<Beat> = vec![];
    _blk3.push(Beat { swar: Some(Pitch::new("PA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk3.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk3.push(Beat { swar: Some(Pitch::new("DHA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk3.push(Beat { swar: Some(Pitch::new("PA".to_string())), long: 3*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk3));

    let mut _blk4: Vec<Beat> = vec![];
    _blk4.push(Beat { swar: Some(Pitch::new("GA".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk4.push(Beat { swar: Some(Pitch::new("RE".to_string())), long: swars::BASE_SWAR_INTERVAL });
    _blk4.push(Beat { swar: Some(Pitch::default()), long: 3*swars::BASE_SWAR_INTERVAL });
    _pakad.push(SwarBlock(_blk4));

    Raag::new("Bhupali".to_string(), _aroha, _avroha, _pakad)
}
