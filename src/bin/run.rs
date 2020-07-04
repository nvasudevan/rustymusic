use rand;
use rand::{Rng, random};
use rodio::{Sink, source::SineWave, default_output_device, output_devices, Device};
use rustymusic::raagas::{elements, bhupali, durga};
use rustymusic::raagas::swars::Pitch;
use rustymusic::raagas::elements::Melody;

fn rand_next() -> Option<u8> {
    let mut rnd_num = rand::thread_rng();
    let n: u8 = rnd_num.gen();

    Some(n)
}

fn main() {
    println!("music test");
    // loop {
    //     let n =  rand_next().unwrap();
    //     println!("rand: {}", n);
    //     if n > 150 {
    //         break
    //     }
    // }

    let dev = default_output_device().unwrap();
    // play_sargam(&dev);

    // let bpli = bhupali::bhupali();
    // bpli.play(&dev);

    let drg = durga::durga();
    drg.play(&dev);
}