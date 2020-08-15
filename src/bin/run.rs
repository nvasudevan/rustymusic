extern crate getopts;

use std::env;

use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use rodio::{decoder, default_output_device, Source};
use rustymusic::raagas::elements::elements;
use rustymusic::raagas::opts;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev = default_output_device().unwrap();
    pub const VOL: f32 = 0.5;

    let opts = opts::build_opts();
    let comp = opts::parse_opts(&opts, args);
    let audio_dev = elements::AudioDevice::new(dev, VOL);
    let beat_source = play_raw_beats_forever(elements::BEATMP3);

    match comp {
        Ok(r) => r.play(&audio_dev, Some(beat_source), 1),
        Err(e) => opts::print_usage(&e.to_string(), &opts),
    }
}
