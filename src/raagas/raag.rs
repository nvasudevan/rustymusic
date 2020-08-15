extern crate yaml_rust;

use crate::raagas::elements::elements::{Pitch, Swar, CONF_DIR};
use crate::raagas::elements::raag::Raag;
use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::elements::swarmaalika::{Antara, Sthayi, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;

use self::yaml_rust::{yaml, Yaml};

fn to_swars(s: &str) -> Vec<Swar> {
    let mut _blk: Vec<Swar> = vec![];
    let swars: Vec<String> = s
        .trim()
        .split(" ")
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
                }
                None => {
                    // TODO: no previous swar, so we should be playing just thaalam.
                }
            }
        } else {
            if sw.contains(":") {
                let _swrs: Vec<String> = sw.split(":").map(|x| x.to_string()).collect();
                let beat_cnt: f32 = 1.0 / _swrs.len() as f32;
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

fn swar_line(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    let mut blk: Vec<SwarBlock> = Vec::new();
    match doc {
        yaml::Yaml::Array(ref v) => {
            let line = v.get(0).unwrap();
            let blks_s: Vec<&str> = line.as_str().unwrap().split(",").collect();
            for _s in blks_s {
                blk.push(SwarBlock(to_swars(_s)));
            }
            Some(blk)
        }
        _ => None,
    }
}

fn aroha_avroha(doc: &Yaml, comp: &str) -> Option<Vec<SwarBlock>> {
    swar_line(&doc[comp])
}

fn pakad(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    swar_line(&doc["pakad"])
}

fn alankars(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    swar_line(&doc["alankars"])
}

fn sthayi(doc: &Yaml) -> Sthayi {
    let line_a = swar_line(&doc["lineA"]);
    let line_b = swar_line(&doc["lineB"]);
    let line_c = swar_line(&doc["lineC"]);

    Sthayi::new(line_a, line_b, line_c)
}

fn antara(doc: &Yaml) -> Antara {
    let line_c = swar_line(&doc["lineC"]);
    let line_d = swar_line(&doc["lineD"]);
    let line_e = swar_line(&doc["lineE"]);

    Antara::new(line_c, line_d, line_e)
}

fn tihayi(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    swar_line(&doc["tihayi"])
}

fn swarmaalika(doc: &Yaml) -> Option<Swarmaalika> {
    let sthayi_s = &doc["sthayi"];
    let antara_s = &doc["antara"];
    let sthayi: Sthayi = sthayi(sthayi_s);
    let antara: Antara = antara(antara_s);
    let tihayi_s = &doc["tihayi"];
    let tihayi = tihayi(tihayi_s);

    Some(Swarmaalika::new(None, sthayi, antara, tihayi))
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

            Some(Raag::new(
                name,
                Some(aroha),
                Some(avroha),
                Some(pakad),
                Some(alankars),
                swarmaalika,
            ))
        }
        _ => None,
    }
}
