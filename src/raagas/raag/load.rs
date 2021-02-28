extern crate yaml_rust;

use rodio::{decoder, Source};
use yaml_rust::YamlLoader;

use self::yaml_rust::{yaml, Yaml};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::time::Duration;
use crate::raagas::swars::{Swar, BeatSrc};
use crate::raagas::constants::{KAN_SWAR_BEAT_COUNT, CONF_DIR, BEAT_MP3};
use crate::raagas::sound::Pitch;
use crate::raagas::swarmaalika::{Sthayi, Antara, Swarmaalika};
use crate::raagas::raag::raag::Raag;
use crate::raagas::utils;
use crate::raagas::swarblock::{SwarBlocks, SwarBlock};
use crate::raagas::swarbeat::SwarBeat;

// traversing from the last item, returns the index of the swarbeat with a swar in it
// S - - M - -, will return 3 (as the last swarbeat is a '-' and M is the one with swar
// from the back
pub fn get_last_swarbeat_with_swar(swarbeats: &mut Vec<SwarBeat>) -> Option<usize> {
    // (0..n) is excluded and we only need 0 to n-1
    for i in (0..swarbeats.len()).rev() {
        let sw_bt = swarbeats.get(i).unwrap();
        if sw_bt.len() > 0 {
            return Some(i);
        }
    }

    None
}

// increment the last swar of previous swarbeat
pub fn extend_last_swar(swarbeats: &mut Vec<SwarBeat>, beat_count_inc: f32) {
    if let Some(i) = get_last_swarbeat_with_swar(swarbeats) {
        if let Some(prev_sw_bt) = swarbeats.get_mut(i) {
            prev_sw_bt.increment_swar_at(prev_sw_bt.len()-1, beat_count_inc);
        }
    }
}

pub fn to_swarbeats(s: &str) -> Vec<SwarBeat> {
    let mut swarbeats_vec: Vec<SwarBeat> = vec![];
    let swarbeats: Vec<String> = s.trim().split(" ").map(|x| x.to_string()).collect();
    for sw_bt in swarbeats {
        let mut swars = Vec::<Swar>::new();
        if sw_bt.eq("-") {
            // S:G -  (G will be a beat and a half)
            // S - -  Go all the way back to S and extend it by 1 beat
            // add an extra beat to the previous swarbeat
            // and nothing to add for current swarbeat
            extend_last_swar(&mut swarbeats_vec, 1.0);
        } else {
            if sw_bt.contains(":") {
                // four cases: either two swars or four swars
                // two swars: S:S, -:S :S S: we don't need 'S:- M' as we can always write it as
                // 'S:M -'
                // four swars: S:S:S:S
                let sw_bts_vec: Vec<String> = sw_bt.split(":").map(|x| x.to_string()).collect();
                if sw_bts_vec.len() == 2 {
                    // first and second swar are each 0.5 beat
                    let first_swar_s = sw_bts_vec.first().unwrap();
                    let last_swar_s = sw_bts_vec.last().unwrap();
                    if first_swar_s.eq("-") {
                        // modify last swar of previous SwarBeat
                        extend_last_swar(&mut swarbeats_vec, 0.5);
                        // and last_swar to swars
                        let last_swar = Swar::new(Pitch::new(last_swar_s.to_string()), 0.5);
                        swars.push(last_swar);
                    } else if last_swar_s.eq("-") {
                        // add the first swar
                        let first_swar = Swar::new(Pitch::new(last_swar_s.to_string()), 1.0);
                        swars.push(first_swar);
                    } else {
                        if first_swar_s.eq("") {
                            swars.push(Swar::empty(0.5));
                        } else {
                            swars.push(Swar::new(
                                Pitch::new(first_swar_s.to_string()),
                                0.5)
                            );
                        }
                        if last_swar_s.eq("") {
                            swars.push(Swar::empty(0.5));
                        } else {
                            swars.push(Swar::new(
                                Pitch::new(last_swar_s.to_string()),
                                0.5)
                            );
                        }
                    }
                } else if sw_bts_vec.len() == 4 {
                    // each of the four swars are 0.25 beat
                    let beat_count = 0.25;
                    for sw in sw_bts_vec {
                        if sw.eq("-")  {
                            extend_last_swar(&mut swarbeats_vec, beat_count);
                        } else {
                            let swar = Swar::new(
                                Pitch::new(sw.to_string()), beat_count
                            );
                            swars.push(swar);
                        }
                    }
                }
            } else if sw_bt.contains("/") {
                // kan swar
                // e.g.: P/M
                let swrs: Vec<String> = sw_bt.split("/").map(|x| x.to_string()).collect();
                let kan = swrs.get(0).unwrap();
                let kan_bt_cnt: f32 = 1.0 * KAN_SWAR_BEAT_COUNT;
                swars.push(Swar::new(Pitch::new(kan.to_string()), kan_bt_cnt));
                let main_swar = swrs.get(1).unwrap();
                swars.push(Swar::new(Pitch::new(main_swar.to_string()), 1.0 - kan_bt_cnt));

            } else {
                // all else, just a plain swar (e.g.: S)
                swars.push(Swar::new(Pitch::new(sw_bt.to_string()), 1.0));
            }
        }
        swarbeats_vec.push(SwarBeat::new(swars));
    }

    swarbeats_vec
}

fn swar_line(doc: &Yaml) -> Option<SwarBlocks> {
    let mut blk: Vec<SwarBlock> = Vec::new();
    match doc {
        yaml::Yaml::Array(ref v) => match v.get(0) {
            Some(line) => {
                if line.is_null() {
                    None
                } else {
                    let blks_s: Vec<&str> = line.as_str().unwrap().split(",").collect();
                    for blk_s in blks_s {
                        blk.push(SwarBlock(to_swarbeats(blk_s)));
                    }
                    Some(SwarBlocks(blk))
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn aroha_avroha(doc: &Yaml, comp: &str) -> Option<SwarBlocks> {
    swar_line(&doc[comp])
}

fn pakad(doc: &Yaml) -> Option<SwarBlocks> {
    swar_line(&doc["pakad"])
}

fn alankars(doc: &Yaml) -> Option<SwarBlocks> {
    swar_line(&doc["alankars"])
}

fn sthayi(doc: &Yaml) -> Option<Sthayi> {
    match doc {
        yaml::Yaml::Array(ref v) => {
            let parse = |i: usize, s: &str| {
                let _y = v.get(i);
                return match _y {
                    Some(line) => swar_line(&line[s]),
                    _ => None,
                };
            };
            let mut lines: HashMap<String, SwarBlocks> = HashMap::new();
            let tags: Vec<&str> = vec!["lineA", "lineB", "lineC"];

            for t in tags {
                for i in 0..v.len() {
                    let line = parse(i, t);
                    match line {
                        Some(blk) => {
                            lines.insert(t.to_string(), blk);
                            break;
                        }
                        _ => {}
                    }
                }
            }

            Some(Sthayi::new(lines))
        }
        _ => None,
    }
}

fn antara(doc: &Yaml) -> Option<Antara> {
    match doc {
        yaml::Yaml::Array(ref v) => {
            let parse = |i: usize, s: &str| {
                let _y = v.get(i);
                return match _y {
                    Some(line) => swar_line(&line[s]),
                    _ => None,
                };
            };

            let mut lines: HashMap<String, SwarBlocks> = HashMap::new();
            let tags: Vec<&str> = vec!["lineC", "lineD", "lineE"];

            for t in tags {
                for i in 0..v.len() {
                    let line = parse(i, t);
                    match line {
                        Some(blk) => {
                            lines.insert(t.to_string(), blk);
                            break;
                        }
                        _ => {}
                    }
                }
            }

            Some(Antara::new(lines))
        }
        _ => None,
    }
}

fn mukra(doc: &Yaml) -> Option<SwarBlocks> {
    swar_line(doc)
}

fn tihayi(doc: &Yaml) -> Option<SwarBlocks> {
    swar_line(doc)
}

fn parse_usize(doc: &Yaml) -> Option<usize> {
    match doc {
        yaml::Yaml::Integer(ref n) => {
            let _n: usize = *n as usize;
            Some(_n)
        }
        _ => None,
    }
}

fn swarmaalika(doc: &Yaml) -> Option<Swarmaalika> {
    let swarmaalika = &doc["swarmaalika"];
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
            let tihayi = tihayi(tihayi_s);

            Some(Swarmaalika::new(mukra, sthayi, antara, tihayi, sam))
        }
        _ => None,
    }
}

fn play_raw_beats_forever(beatp: (&str, f32)) -> BeatSrc {
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

pub fn load_yaml(raag: &str, composition: &str) -> Option<Raag> {
    // Given a raag name returns a Raag
    let comp_path = format!("{}.yaml", composition);
    let raagp = Path::new(CONF_DIR).join(raag).join(comp_path);
    // let raagp = format!("{}/{}/{}.yaml", CONF_DIR, raag, composition);
    let s = utils::file_as_str(raagp);
    let yamlldr = YamlLoader::load_from_str(&s);
    match &yamlldr {
        Ok(docs) => {
            let doc = &docs[0];
            let aroha = aroha_avroha(&doc, "aroha")?;
            let avroha = aroha_avroha(&doc, "avroha")?;
            let pakad = pakad(&doc)?;
            let alankars = match alankars(&doc) {
                Some(v) => Some(v),
                _ => None,
            };
            let swarmaalika = swarmaalika(&doc)?;
            let beat_src = play_raw_beats_forever(BEAT_MP3);

            Some(Raag::new(
                raag.to_string(),
                Some(aroha),
                Some(avroha),
                Some(pakad),
                alankars,
                swarmaalika,
                Some(beat_src),
            ))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::raagas::raag::load;
    use crate::raagas::swarblock::SwarBlock;

    /// test no of swarbeats match for a sequence
    #[test]
    fn test_load_swarbeats_count() {
        let s = "S:R:M:P S - - M P:P -:D :D P/M";
        let swarbeats = load::to_swarbeats(s);
        assert_eq!(swarbeats.len(), 9);
    }

    /// test if the beat count matches when a swar is extended
    #[test]
    fn test_extend_swar_with_beat() {
        let s = "S - G -";
        let mut swarbeats = load::to_swarbeats(s);
        load::extend_last_swar(&mut swarbeats, 1.0);
        let i = load::get_last_swarbeat_with_swar(&mut swarbeats).unwrap();
        let sw_bt = swarbeats.get(i).unwrap();
        let last_swar = sw_bt.swars.last().unwrap();
        assert_eq!(last_swar.beat_cnt, 3.0);
    }

    /// test if the beat count matches when half a beat swar is extended
    #[test]
    fn test_extend_swar_with_half_beat() {
        let s = "S - :G";
        let mut swarbeats = load::to_swarbeats(s);
        println!("swarbeats: {:?}", swarbeats);
        load::extend_last_swar(&mut swarbeats, 1.0);
        println!("swarbeats: {:?}", swarbeats);
        let i = load::get_last_swarbeat_with_swar(&mut swarbeats).unwrap();
        let sw_bt = swarbeats.get(i).unwrap();
        let last_swar = sw_bt.swars.last().unwrap();
        assert_eq!(last_swar.beat_cnt, 1.5);
    }
}