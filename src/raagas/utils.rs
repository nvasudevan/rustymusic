use std::fs::File;
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn lines_from_file(fp: String) -> Vec<String> {
    let _path = std::path::Path::new(fp.as_str());
    let f = File::open(_path);
    let buf = std::io::BufReader::new(f.unwrap());
    let lines: Vec<String> = buf.lines()
        .map(|x| x.unwrap())
        .collect();

    lines
}

pub fn delay(t: u64) {
    sleep(Duration::from_secs(t));
}
