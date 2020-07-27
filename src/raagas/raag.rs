extern crate yaml_rust;

use crate::raagas::elements::{Raag, Swar, SwarBlock, Pitch, CONF_DIR, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;
use std::error::Error;
use self::yaml_rust::{yaml, Yaml};


fn to_swars(s: &str) -> Vec<Swar> {
    let mut _blk: Vec<Swar> = vec![];
    let swars: Vec<String> = s.trim().split(" ")
        .map(|x| x.to_ascii_uppercase())
        .collect();
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

fn aroha_avroha(doc: &Yaml, comp: &str) -> Option<Vec<Swar>> {
    let _s = &doc[comp];

    match _s {
        yaml::Yaml::Array(ref v) => {
            // aroha/avroha is always in one line
            assert_eq!(v.len(), 1);
            let line = v.get(0).unwrap();
            let swars = to_swars(line.as_str().unwrap());
            Some(swars)
        },
        _ => {
            None
        }
    }
}

fn pakad(doc: &Yaml) -> Vec<SwarBlock> {
    let pakad_s = &doc["pakad"];
    let mut pakad: Vec<SwarBlock> = Vec::new();
    match pakad_s {
        yaml::Yaml::Array(ref v) => {
            for line in v {
                let blks: Vec<&str> = line.as_str().unwrap().split(",").collect();
                for blk in blks {
                    let _blk = to_swars(blk);
                    pakad.push(SwarBlock(_blk));
                }
            }
        },
        _ => {}
    }

    pakad
}

fn alankars(doc: &Yaml) -> Vec<Vec<Swar>> {
    let alanakars_s = &doc["alankars"];
    let mut alankars: Vec<Vec<Swar>> = Vec::new();
    match alanakars_s {
        yaml::Yaml::Array(ref v) => {
            for line in v {
                let alankar = to_swars(line.as_str().unwrap());
                alankars.push(alankar);
            }
        },
        _ => {}
    }

    alankars
}

fn swarmaalika(doc: &Yaml) -> Swarmaalika {
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

    Swarmaalika::new(sthayi, antara)
}

pub fn raag(name: String) -> Option<Raag> {
    let raagp = format!("{}/{}.yaml", CONF_DIR, name);
    let s = utils::file_as_str(raagp);
    let yamlldr = YamlLoader::load_from_str(&s);
    match &yamlldr {
        Ok(docs) => {
            let doc = &docs[0];
            let aroha = aroha_avroha(&doc, "aroha").unwrap();
            let avroha = aroha_avroha(&doc, "avroha").unwrap();
            let pakad = pakad(&doc);
            let alankars = alankars(&doc);
            let swarmaalika = swarmaalika(&doc);

            let r = Raag::new(name, aroha, avroha, pakad, alankars, swarmaalika);
            println!("r: {:#?}", r);

            Some(r)
        },
        _ => {
            None
        }
    }

}