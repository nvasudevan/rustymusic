#[cfg(test)]
mod tests {
    use crate::raagas::elements;
    use crate::raagas::raag;
    use crate::raagas::random;

    #[test]
    fn test_hertz_sub_op() {
        let base_hz = elements::Hertz(440.0);
        assert_eq!(elements::Hertz(439.0), base_hz - elements::Hertz(1.0))
    }

    #[test]
    fn test_base_pitch_value() {
        let base_hz = elements::Hertz(277.18);
        let p = elements::Pitch::default().hertz().unwrap();
        assert_eq!(base_hz, p);
    }

    #[test]
    fn test_base_swar_is_sa() {
        let base_hz = elements::Hertz(277.18);
        let sa_pitch = elements::Pitch::new("SA".to_string());
        let sa = elements::Swar::new(sa_pitch, 1);
        assert_eq!(sa.pitch.unwrap().hertz().unwrap(), base_hz);
    }

    #[test]
    fn test_swar_single_beat() {
        // test string version of swar
        let sa_pitch = elements::Pitch::new("sa".to_string());
        let sa = elements::Swar::new(sa_pitch, 1);
        assert_eq!(sa.to_string(), "sa");
    }

    #[test]
    fn test_swar_multiple_beats() {
        // test string version of swar
        let sa_pitch = elements::Pitch::new("sa".to_string());
        let sa = elements::Swar::new(sa_pitch, 3);
        assert_eq!(sa.to_string(), "sa -  - ");
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
        let expected = " SA RE MA PA DHA SA. - ";

        let aroha = raag.aroha();
        let mut aroha_s = String::new();
        for sw in aroha {
            let mut dash: String = "".to_string();
            if sw.beat_cnt > 1 {
                dash = (1..sw.beat_cnt-1).map(|_|  " - ").collect::<String>();
            }
            aroha_s = format!("{} {}{}", aroha_s, sw, dash);
        }
        assert_eq!(aroha_s, expected);
    }

    #[test]
    fn test_raag_avroha() {
        let s = "durga";
        let raag = raag::raag(s.to_string()).unwrap();
        let expected: Vec<elements::Swar> = vec![
            elements::Swar::new(elements::Pitch::new("SA.".to_string()), 1),
            elements::Swar::new(elements::Pitch::new("DHA".to_string()), 1),
            elements::Swar::new(elements::Pitch::new("PA".to_string()), 1),
            elements::Swar::new(elements::Pitch::new("MA".to_string()), 1),
            elements::Swar::new(elements::Pitch::new("RE".to_string()), 1),
            elements::Swar::new(elements::Pitch::new("SA".to_string()), 2),
        ];

        let avroha = raag.avroha().clone();
        assert_eq!(avroha, expected);
    }

    #[test]
    fn test_randomiser() {
        let z: usize = 3;
        // test 3 notes are returned
        let swars = random::randomiser(z);
        assert_eq!(swars.len(), z);
    }
}
