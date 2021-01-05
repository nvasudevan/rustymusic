#[cfg(test)]
mod tests {
    use crate::raagas::elements::elements::{Hertz, Pitch, Swar};
    use crate::raagas::raag;
    use crate::raagas::random;
    use crate::SWARS;

    use crate::raagas::elements::swarblock::SwarBlock;

    #[test]
    fn test_base_swar_is_sa() {
        let base_hz = Hertz::new(277.18, "C#".to_string());
        let sa_pitch = Pitch::new("S".to_string());
        let sa = Swar::new(sa_pitch, 1.0);
        assert_eq!(sa.pitch.unwrap().hertz().unwrap(), base_hz);
    }

    #[test]
    fn test_swar_single_beat() {
        // test string version of swar
        let sa_pitch = Pitch::new("S".to_string());
        let sa = Swar::new(sa_pitch, 1.0);
        assert_eq!(sa.to_string(), "S[C#] ");
    }

    #[test]
    fn test_swar_multiple_beats() {
        // test string version of swar
        let sa_pitch = Pitch::new("S".to_string());
        let sa = Swar::new(sa_pitch, 3.0);
        assert_eq!(sa.to_string(), "S[C#] -  - ");
    }

    #[test]
    fn test_raag() {
        let s = "durga";
        let raag = raag::raag(s.to_string()).unwrap();
        assert_eq!(raag.name(), s);
    }

    #[test]
    fn test_raag_aroha_as_string() {
        let s = "durga";
        let raag = raag::raag(s.to_string()).unwrap();
        let expected = "S[C#] - R[D#] - M[F#] - P[G#] - D[A#] - S.[C#] -  - ";

        let mut aroha_s = String::new();
        match raag.aroha() {
            Some(aroha) => {
                for blk in aroha {
                    for sw in &blk.0 {
                        aroha_s = format!("{}{}", aroha_s, sw);
                    }
                }
                assert_eq!(aroha_s, expected);
            }
            _ => {}
        }
    }

    #[test]
    fn test_raag_avroha() {
        let s = "durga";
        let raag = raag::raag(s.to_string()).unwrap();
        let swars: Vec<Swar> = vec![
            Swar::new(Pitch::new("S.".to_string()), 2.0),
            Swar::new(Pitch::new("D".to_string()), 2.0),
            Swar::new(Pitch::new("P".to_string()), 2.0),
            Swar::new(Pitch::new("M".to_string()), 2.0),
            Swar::new(Pitch::new("R".to_string()), 2.0),
            Swar::new(Pitch::new("S".to_string()), 3.0),
        ];
        let expected: Vec<SwarBlock> = vec![SwarBlock(swars)];

        match raag.avroha() {
            Some(avroha) => {
                assert_eq!(avroha.clone(), expected);
            }
            _ => {}
        }
    }

    #[test]
    fn test_randomiser() {
        let z: usize = 3;
        // test 3 notes are returned
        let _swars: Vec<String> = SWARS.keys().map(|x| x.to_string()).collect();
        let s = "durga";
        let raag = raag::raag(s.to_string()).unwrap();
        let swars = random::randomiser(&raag, z);
        let _swars = swars.unwrap();
        println!("_swars: {:?}", _swars);
        assert_eq!(_swars.len(), z);
    }
}
