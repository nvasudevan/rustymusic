extern crate yaml_rust;

use crate::raagas::elements::{Raag, Swar, SwarBlock, Pitch, CONF_DIR, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;
use std::error::Error;


fn aroha(fp: String) -> Vec<Swar> {
    let lines = utils::lines_from_file(fp);
    let mut aroha: Vec<Swar> = vec![];
    for l in lines {
        let swars: Vec<String> = l.split(" ")
                                  .map(|x| x.to_ascii_uppercase())
                                  .collect();
        for sw in swars {
            if sw.eq("-") {
                //add an extra beat to previous swar
                let prev = aroha.pop();
                match &prev {
                    Some(_sw) => {
                        let p = _sw.pitch.as_ref().unwrap();
                        aroha.push(Swar::new(p.clone(), _sw.beat_cnt + 1))
                    },
                    None => {
                        // TODO: no previous swar, so we should be playing just thaalam.
                    }
                }
            } else {
                aroha.push(Swar::new(Pitch::new(sw), 1))
            }
        }
    }

    aroha
}

fn avroha(fp: String) -> Vec<Swar> {
    let lines = utils::lines_from_file(fp);
    let mut avroha: Vec<Swar> = vec![];
    for l in lines {
        let swars: Vec<String> = l.split(" ")
            .map(|x| x.to_ascii_uppercase())
            .collect();
        for sw in swars {
            if sw.eq("-") {
                //add an extra beat to previous swar
                let prev = avroha.pop();
                match &prev {
                    Some(_sw) => {
                        let p = _sw.pitch.as_ref().unwrap();
                        avroha.push(Swar::new(p.clone(), _sw.beat_cnt + 1))
                    },
                    None => {
                        // TODO: no previous swar, so we should be playing just thaalam.
                    }
                }
            } else {
                avroha.push(Swar::new(Pitch::new(sw), 1))
            }
        }
    }

    avroha
}

fn pakad(fp: String) -> Vec<SwarBlock> {
    let lines = utils::lines_from_file(fp);
    let mut pakad: Vec<SwarBlock> = vec![];
    for l in lines {
        let blks: Vec<String>  = l.split(",")
                                  .map(|x| x.trim().to_ascii_uppercase())
                                  .collect();
        for blk in blks {
            let mut _blk: Vec<Swar> = vec![];
            let swars: Vec<String> = blk.split(" ").map(|x| x.to_string()).collect();
            for swar in swars {
                if swar.eq("-") {
                    let prev = _blk.pop().unwrap();
                    let beat_cnt = prev.beat_cnt + 1;

                    _blk.push(Swar::new(prev.pitch.unwrap(), beat_cnt));
                } else {
                    _blk.push(Swar::new(Pitch::new(swar), 1));
                }
            }

            pakad.push(SwarBlock(_blk));
        }
    }

    pakad
}

fn alankars(fp: String) -> Vec<Vec<Swar>> {
    let lines = utils::lines_from_file(fp);
    let mut alankars: Vec<Vec<Swar>> = Vec::new();
    for line in lines {
        let swars: Vec<String> = line.split(" ")
                                     .map(|x| x.to_ascii_uppercase())
                                     .collect();
        let mut alankar: Vec<Swar> = vec![];
        for swar in swars {
            alankar.push(Swar { pitch: Some(Pitch::new(swar)), beat_cnt: 1 });
        }

        alankars.push(alankar);
    }

    alankars
}

//-> Result<Swarmaalika, dyn Error> {
fn swarmaalika(fp: String) -> Swarmaalika {
    let s = utils::file_as_str(fp);
    let yamlldr = YamlLoader::load_from_str(&s);
    // match &yamlldr {
    //     Ok(docs) => {
    //         let doc = &docs[0];
    //         let sthayi_s = &doc["sthayi"];
    //         let antara_s = &doc["antara"];
    //         let sthayi: Vec<SwarBlock> = Vec::new();
    //         let antara: Vec<SwarBlock> = Vec::new();
    //         let swarmaalika = Swarmaalika::new(sthayi, antara);
    //
    //         Ok()
    //     },
    //     _ => {
    //         None
    //     }
    // }

    Swarmaalika::new(Vec::new(), Vec::new())
}

pub fn raag(name: String) -> Raag {
    let arohap = format!("{}/{}/aroha.txt", CONF_DIR, name);
    let aroha = aroha(arohap);

    let avrohap = format!("{}/{}/avroha.txt", CONF_DIR, name);
    let avroha = avroha(avrohap);

    let pakadp = format!("{}/{}/pakad.txt", CONF_DIR, name);
    let pakad = pakad(pakadp);

    let alankarsp = format!("{}/{}/alankars.txt", CONF_DIR, name);
    let alankars = alankars(alankarsp);

    let swarmaalikap = format!("{}/{}/swarmaalika.yaml", CONF_DIR, name);
    let swarmaalika = swarmaalika(swarmaalikap);

    Raag::new(name, aroha, avroha, pakad, alankars, swarmaalika)
}