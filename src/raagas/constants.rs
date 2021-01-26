use std::collections::HashMap;
use crate::raagas::sound::Hertz;

pub(crate) const RAAGAS: [&str;9] = [
    "durga", "yaman", "bhupali", "hamsadhwani",
    "yeri_aali", "bairav", "bhairavi", "daesh", "malkauns"
];
pub const BPS: f32 = 0.5; // equivalent to 120 BPM
pub const CONF_DIR: &str = "./config";
pub const BEAT_MP3: (&str, f32) = ("./samples/1beat.mp3", BPS);
pub const TIHAYI_TIMES: i8 = 3;
pub const KAN_SWAR_BEAT_COUNT: f32 = 0.2;
pub const VOL: f32 = 0.5;
pub const PLAY_PAUSE_DURATION: f32 = 2.0;

lazy_static! {
    pub static ref SWARS: HashMap<&'static str, Hertz> = initialise_swars();
}

pub fn initialise_swars<'a>() -> HashMap<&'a str, Hertz> {
    let mut swars: HashMap<&str, Hertz> = HashMap::new();
    swars.insert(".P", Hertz::new(207.65, "G#".to_string()));
    swars.insert(".d", Hertz::new(220.00, "A".to_string()));
    swars.insert(".D", Hertz::new(233.08, "A#".to_string()));
    swars.insert(".n", Hertz::new(246.94, "B".to_string()));
    swars.insert(".N", Hertz::new(261.63, "C".to_string()));

    swars.insert("S", Hertz::new(277.18, "C#".to_string()));
    swars.insert("r", Hertz::new(293.66, "D".to_string()));
    swars.insert("R", Hertz::new(311.13, "D#".to_string()));
    swars.insert("g", Hertz::new(329.63, "E".to_string()));
    swars.insert("G", Hertz::new(349.23, "F".to_string()));
    swars.insert("M", Hertz::new(369.99, "F#".to_string()));
    swars.insert("M'", Hertz::new(392.00, "G".to_string()));
    swars.insert("P", Hertz::new(415.30, "G#".to_string()));
    swars.insert("d", Hertz::new(440.0, "A".to_string()));
    swars.insert("D", Hertz::new(466.16, "A#".to_string()));
    swars.insert("n", Hertz::new(493.88, "B".to_string()));
    swars.insert("N", Hertz::new(523.25, "C".to_string()));
    swars.insert("S.", Hertz::new(554.37, "C#".to_string()));
    swars.insert("r.", Hertz::new(587.33, "D".to_string()));
    swars.insert("R.", Hertz::new(622.25, "D#".to_string()));
    swars.insert("g.", Hertz::new(659.25, "E".to_string()));
    swars.insert("G.", Hertz::new(698.46, "F".to_string()));
    swars.insert("M.", Hertz::new(739.99, "F#".to_string()));
    swars.insert("M'.", Hertz::new(783.99, "G".to_string()));

    swars
}

