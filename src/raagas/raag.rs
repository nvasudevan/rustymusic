extern crate yaml_rust;

use crate::raagas::elements::{Raag, Swar, SwarBlock, Pitch, CONF_DIR, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;

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
                    _blk.push(Swar::new(p.clone(), _sw.beat_cnt + 1.0))
                },
                None => {
                    // TODO: no previous swar, so we should be playing just thaalam.
                }
            }
        } else {
            if sw.contains(":") {
                let _swrs: Vec<String> = sw.split(":").map(|x| x.to_string()).collect();
                let beat_cnt: f32 = 1.0/_swrs.len() as f32;
                for _sw in _swrs {
                    _blk.push(Swar::new(Pitch::new(_sw), beat_cnt));
                }
            } else {
                _blk.push(Swar::new(Pitch::new(sw), 1.0));
            }
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

fn pakad(doc: &Yaml) -> Option<Vec<SwarBlock>> {
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

            Some(pakad)
        },
        _ => {
            None
        }
    }
}

fn alankars(doc: &Yaml) -> Option<Vec<Vec<Swar>>> {
    let alanakars_s = &doc["alankars"];
    let mut alankars: Vec<Vec<Swar>> = Vec::new();
    match alanakars_s {
        yaml::Yaml::Array(ref v) => {
            for line in v {
                let alankar = to_swars(line.as_str().unwrap());
                alankars.push(alankar);
            }
            Some(alankars)
        },
        _ => {
            None
        }
    }
}

fn swarmaalika(doc: &Yaml) -> Option<Swarmaalika> {
    let sthayi_s = &doc["sthayi"];
    let antara_s = &doc["antara"];
    let mut sthayi: Vec<SwarBlock> = Vec::new();
    let mut antara: Vec<SwarBlock> = Vec::new();
    match sthayi_s {
        yaml::Yaml::Array(ref v) => {
            for line in v {
                let _blk = to_swars(line.as_str()
                                          .expect(&format!("{:?} can't be converted to swars", line)));
                sthayi.push(SwarBlock(_blk));
            }
        },
        _ => {}
    }
    match antara_s {
        yaml::Yaml::Array(ref v) => {
             for line in v {
                 let _blk = to_swars(line.as_str()
                                           .expect(&format!("{:?} can't be converted to swars", line)));
                 antara.push(SwarBlock(_blk));
             }
        },
        _ => {}
    }

    Some(Swarmaalika::new(sthayi, antara))
}

pub fn raag(name: String) -> Option<Raag> {
    // Given a raag name returns a Raag
    let raagp = format!("{}/{}.yaml", CONF_DIR, name);
    let s = utils::file_as_str(raagp);
    let yamlldr = YamlLoader::load_from_str(&s);
    match &yamlldr {
        Ok(docs) => {
            let doc = &docs[0];
            let aroha = aroha_avroha(&doc, "aroha")?;
            let avroha = aroha_avroha(&doc, "avroha")?;
            let pakad = pakad(&doc)?;
            let alankars = alankars(&doc)?;
            let swarmaalika = swarmaalika(&doc)?;

            Some(Raag::new(name, aroha, avroha, pakad, alankars, swarmaalika))
        },
        _ => {
            None
        }
    }
}