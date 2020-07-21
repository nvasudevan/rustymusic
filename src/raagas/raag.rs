use std::path::Path;
use crate::raagas::swars;
use crate::raagas::elements::{Raag, Beat, SwarBlock};
use crate::raagas::swars::Pitch;

fn aroha(fp: &Path) -> Vec<Beat> {
    let s = std::fs::read_to_string(fp).unwrap();
    let _s = s.replace("\n", "");
    let swars: Vec<String> = _s.split(" ").map(|x| x.to_ascii_uppercase()).collect();

    let mut _aroha: Vec<Beat> = vec![];
    for swar in swars {
        if swar.eq("SA+") {
            _aroha.push(Beat { swar: Some(Pitch::new(swar)), long: 2*swars::BASE_SWAR_INTERVAL });
        } else {
            _aroha.push(Beat { swar: Some(Pitch::new(swar)), long: swars::BASE_SWAR_INTERVAL });
        }
    }

    _aroha
}

fn avroha(fp: &Path) -> Vec<Beat> {
    let s = std::fs::read_to_string(fp).unwrap();
    let _s = s.replace("\n", "");
    let swars: Vec<String> = _s.split(" ").map(|x| x.to_ascii_uppercase()).collect();

    let mut _avroha: Vec<Beat> = vec![];
    for swar in swars {
        if swar.eq("SA") {
            _avroha.push(Beat { swar: Some(Pitch::new(swar)), long: 2*swars::BASE_SWAR_INTERVAL });
        } else {
            _avroha.push(Beat { swar: Some(Pitch::new(swar)), long: swars::BASE_SWAR_INTERVAL });
        }
    }

    _avroha
}

fn pakad(fp: &Path) -> Vec<SwarBlock> {
    let s = std::fs::read_to_string(fp).unwrap();
    let blks: Vec<String>  = s.split(",").map(|x| x.trim().to_ascii_uppercase()).collect();
    let mut _pakad: Vec<SwarBlock> = vec![];

    for blk in blks {
        let mut _blk: Vec<Beat> = vec![];
        let swars: Vec<String> = blk.split(" ").map(|x| x.to_string()).collect();
        for swar in swars {
            if swar.eq("-") {
                let prev = _blk.pop().unwrap();
                let long = prev.long + 1;

                _blk.push(Beat { swar: prev.swar, long: long });
            } else {
                _blk.push(Beat { swar: Some(Pitch::new(swar)), long: swars::BASE_SWAR_INTERVAL });
            }
        }

        _pakad.push(SwarBlock(_blk));
    }

    _pakad
}

pub fn raag(name: String, fp: String) -> Raag {
    let arohap = format!("{}/aroha.txt", fp);
    let aroha_path = std::path::Path::new(arohap.as_str());
    let arohav = aroha(&aroha_path);

    let avrohap = format!("{}/avroha.txt", fp);
    let avroha_path = std::path::Path::new(avrohap.as_str());
    let avrohav = avroha(&avroha_path);

    let pakadp = format!("{}/pakad.txt", fp);
    let pakad_path = std::path::Path::new(pakadp.as_str());
    let pakadv = pakad(&pakad_path);

    Raag::new(name, arohav, avrohav, pakadv)
}