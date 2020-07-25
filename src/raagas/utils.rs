use std::fs::File;
use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn lines_from_file(fp: String) -> Vec<String> {
    // println!("reading from file: {}", fp);
    let _path = std::path::Path::new(fp.as_str());
    let f = File::open(_path);
    let buf = std::io::BufReader::new(f.unwrap());
    let lines: Vec<String> = buf.lines()
        .map(|x| x.unwrap())
        .collect();

    lines
}

pub(crate) fn file_as_str(fp: String) -> String {
    let f = std::fs::read_to_string(fp);
    match &f {
        Ok(_s) => {
            String::from(_s)
        },
        _ => {
            String::new()
        }
    }
}

pub fn delay(t: u64) {
    sleep(Duration::from_secs(t));
}

pub(crate) fn io_flush() {
    match std::io::stdout().flush() {
        Ok(()) => {},
        _ => { panic!("I/O couldn't be flushed to terminal!")}
    }
}
