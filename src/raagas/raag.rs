extern crate yaml_rust;

use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use rodio::{decoder, Source};
use crate::raagas::elements::elements::{Pitch, Swar, BEATMP3, CONF_DIR, KAN_SWAR_BEAT_COUNT};
use crate::raagas::elements::raag::Raag;
use crate::raagas::elements::swarblock::SwarBlock;
use crate::raagas::elements::swarmaalika::{Antara, Sthayi, Swarmaalika};
use crate::raagas::utils;
use yaml_rust::YamlLoader;

use self::yaml_rust::{yaml, Yaml};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn to_swars(s: &str) -> Vec<Swar> {
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
            } else if sw.contains("/") {
                // kan swar
                let _swrs: Vec<String> = sw.split("/").map(|x| x.to_string()).collect();
                let _kan = _swrs.get(0).unwrap();
                let _kan_bt_cnt: f32 = 1.0 * KAN_SWAR_BEAT_COUNT;
                _blk.push(Swar::new(Pitch::new(_kan.to_string()), _kan_bt_cnt));
                let _sw = _swrs.get(1).unwrap();
                _blk.push(Swar::new(Pitch::new(_sw.to_string()), 1.0 - _kan_bt_cnt));
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
            match v.get(0) {
                Some(line) => {
                    if line.is_null() {
                        None
                    } else {
                        println!("line: {:#?}", line);
                        let blks_s: Vec<&str> = line.as_str().unwrap().split(",").collect();
                        for _s in blks_s {
                            blk.push(SwarBlock(to_swars(_s)));
                        }
                        Some(blk)
                    }
                },
                _ => { None }
            }
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

fn sthayi(doc: &Yaml) -> Option<Sthayi> {
    match doc {
        yaml::Yaml::Array(ref v) => {
            let parse = |i: usize, s: &str| {
                let _y = v.get(i);
                return match _y {
                    Some(line) => {
                        swar_line(&line[s])
                    },
                    _ => { None }
                };
            };
            let mut lines: HashMap<String, Vec<SwarBlock>> = HashMap::new();
            let tags: Vec<&str> = vec!["lineA", "lineB", "lineC"];

            for t in tags {
                for i in 0..v.len() {
                    let line = parse(i, t);
                    match line {
                        Some(blk) => {
                            lines.insert(t.to_string(), blk);
                            break
                        },
                        _ => {}
                    }

                }
            }

            Some(Sthayi::new(lines))
        },
        _ => { None }
    }
}

fn antara(doc: &Yaml) -> Option<Antara> {
    match doc {
        yaml::Yaml::Array(ref v) => {
            let parse = |i: usize, s: &str| {
                let _y = v.get(i);
                return match _y {
                    Some(line) => {
                        swar_line(&line[s])
                    },
                    _ => { None }
                };
            };

            let mut lines: HashMap<String, Vec<SwarBlock>> = HashMap::new();
            let tags: Vec<&str> = vec!["lineC", "lineD", "lineE"];

            for t in tags {
                for i in 0..v.len() {
                    let line = parse(i, t);
                    match line {
                        Some(blk) => {
                            lines.insert(t.to_string(), blk);
                            break
                        },
                        _ => {}
                    }

                }
            }

            Some(Antara::new(lines))
        },
        _ => { None }
    }
}

fn mukra(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    swar_line(doc)
}

fn tihayi(doc: &Yaml) -> Option<Vec<SwarBlock>> {
    swar_line(doc)
}

fn parse_usize(doc: &Yaml) -> Option<usize> {
    match doc {
        yaml::Yaml::Integer(ref n) => {
            let _n: usize = *n as usize;
            Some(_n)
        },
        _ => { None }
    }
}

fn swarmaalika(doc: &Yaml) -> Option<Swarmaalika> {
    let swarmaalika = &doc["swarmaalika"];
    println!("swarmaalika: {:#?}", swarmaalika);
    match swarmaalika {
        yaml::Yaml::Array(ref v) => {
            let sam_yaml = v.get(0).unwrap();
            let sam_s = &sam_yaml["sam"];
            let sam = parse_usize(sam_s);

            let mukra_yaml = v.get(1).unwrap();
            let mukra_s = &mukra_yaml["mukra"];
            let mukra = mukra(mukra_s);

            let sthayi_yaml = v.get(2).unwrap();
            let sthayi_s = &sthayi_yaml["sthayi"];
            let sthayi: Sthayi = sthayi(sthayi_s).unwrap();

            let antara_yaml = v.get(3).unwrap();
            let antara_s = &antara_yaml["antara"];
            let antara: Antara = antara(antara_s).unwrap();

            let tihayi_yaml = v.get(4).unwrap();
            let tihayi_s = &tihayi_yaml["tihayi"];
            let tihayi = parse_usize(tihayi_s);

            Some(Swarmaalika::new(mukra, sthayi, antara, tihayi, sam))
        },
        _ => {
            None
        }
    }
}

fn play_raw_beats_forever(beatp: (&str, f32)) -> Repeat<TakeDuration<Decoder<BufReader<File>>>> {
    let f = File::open(beatp.0).expect(&format!("Unable to open file {}", beatp.0));
    let source = decoder::Decoder::new(BufReader::new(f)).unwrap();
    // we are having to do this as the total_duration is returned none for
    // wav, mp3 files in some cases.
    let t = match source.total_duration() {
        Some(_t) => _t,
        _ => Duration::from_secs_f32(beatp.1),
    };

    let beat_src = source.take_duration(t).repeat_infinite();

    beat_src
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
            let beat_src = play_raw_beats_forever(BEATMP3);

            Some(Raag::new(
                name,
                Some(aroha),
                Some(avroha),
                Some(pakad),
                Some(alankars),
                swarmaalika,
                Some(beat_src)
            ))
        }
        _ => None,
    }
}
