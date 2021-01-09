use std::io::BufReader;
use std::fs::File;
use rodio::decoder::Decoder;
use rodio::source::{Repeat, TakeDuration};
use rodio::Sink;

pub struct Taal {
    taal: Repeat<TakeDuration<Decoder<BufReader<File>>>>,
    bps: f32,
}

impl Taal {
    pub fn new(taal: Repeat<TakeDuration<Decoder<BufReader<File>>>>, bps: f32) -> Self {
        Taal { taal, bps }
    }

    pub fn play(&self, sink: Sink) {
        sink.append(self.taal.clone());
    }
}
