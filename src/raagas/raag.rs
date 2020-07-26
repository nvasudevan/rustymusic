extern crate yaml_rust;

use crate::raagas::elements::{Raag, Swar, SwarBlock, Pitch, CONF_DIR, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;
use std::error::Error;
use self::yaml_rust::yaml;


fn to_swars(s: &str) -> Vec<Swar> {
    let mut _blk: Vec<Swar> = vec![];
    let swars: Vec<String> = s.split(" ")
        .map(|x| x.to_ascii_uppercase())
        .collect();
    println!("swars: {:?}", swars);
    for sw in swars {
        if sw.eq("-") {
            //add an extra beat to previous swar
            let prev = _blk.pop();
            match &prev {
                Some(_sw) => {
                    let p = _sw.pitch.as_ref().unwrap();
                    _blk.push(Swar::new(p.clone(), _sw.beat_cnt + 1))
                },
                None => {
                    // TODO: no previous swar, so we should be playing just thaalam.
                }
            }
        } else {
            _blk.push(Swar::new(Pitch::new(sw), 1));
        }
    }

    _blk
}

fn aroha_avroha(fp: String) -> Vec<Swar> {
    let lines = utils::lines_from_file(fp);
    let mut swars = to_swars(lines.get(0).unwrap().as_str());

    swars
}

fn pakad(fp: String) -> Vec<SwarBlock> {
    let lines = utils::lines_from_file(fp);
    let mut pakad: Vec<SwarBlock> = vec![];
    for l in lines {
        let blks: Vec<String>  = l.split(",")
                                  .map(|x| x.trim().to_ascii_uppercase())
                                  .collect();
        for blk in blks {
            let mut _blk = to_swars(blk.as_str());
            pakad.push(SwarBlock(_blk));
        }
    }

    pakad
}

fn alankars(fp: String) -> Vec<Vec<Swar>> {
    let lines = utils::lines_from_file(fp);
    let mut alankars: Vec<Vec<Swar>> = Vec::new();
    for line in lines {
        let alankar = to_swars(line.as_str());
        alankars.push(alankar);
    }

    alankars
}

fn swarmaalika(fp: String) -> Option<Swarmaalika> {
    let s = utils::file_as_str(fp);
    let yamlldr = YamlLoader::load_from_str(&s);
    match &yamlldr {
        Ok(docs) => {
            let doc = &docs[0];
            let sthayi_s = &doc["sthayi"];
            let antara_s = &doc["antara"];
            let mut sthayi: Vec<SwarBlock> = Vec::new();
            let mut antara: Vec<SwarBlock> = Vec::new();
            match sthayi_s {
                yaml::Yaml::Array(ref v) => {
                    for line in v {
                        let _blk = to_swars(line.as_str().unwrap());
                        sthayi.push(SwarBlock(_blk));
                    }
                },
                _ => {}
            }
            match antara_s {
                yaml::Yaml::Array(ref v) => {
                     for line in v {
                         let _blk = to_swars(line.as_str().unwrap());
                         antara.push(SwarBlock(_blk));
                     }
                },
                _ => {}
            }
            let swarmaalika = Swarmaalika::new(sthayi, antara);

            Some(swarmaalika)
        },
        _ => {
            None
        }
    }

}

pub fn raag(name: String) -> Raag {
    let arohap = format!("{}/{}/aroha.txt", CONF_DIR, name);
    let aroha = aroha_avroha(arohap);

    let avrohap = format!("{}/{}/avroha.txt", CONF_DIR, name);
    let avroha = aroha_avroha(avrohap);

    let pakadp = format!("{}/{}/pakad.txt", CONF_DIR, name);
    let pakad = pakad(pakadp);

    let alankarsp = format!("{}/{}/alankars.txt", CONF_DIR, name);
    let alankars = alankars(alankarsp);

    let swarmaalikap = format!("{}/{}/swarmaalika.yaml", CONF_DIR, name);
    let swarmaalika = swarmaalika(swarmaalikap).unwrap();

    Raag::new(name, aroha, avroha, pakad, alankars, swarmaalika)
}