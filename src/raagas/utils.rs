use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;
use std::path::PathBuf;

pub(crate) fn lines_from_file(fp: String) -> Vec<String> {
    // println!("reading from file: {}", fp);
    let _path = std::path::Path::new(fp.as_str());
    let f = std::fs::File::open(_path);
    let buf = std::io::BufReader::new(f.unwrap());
    let lines: Vec<String> = buf.lines().map(|x| x.unwrap()).collect();

    lines
}

pub(crate) fn file_as_str(fp: PathBuf) -> String {
    let f = std::fs::read_to_string(fp);
    match &f {
        Ok(_s) => String::from(_s),
        _ => String::new(),
    }
}

pub fn delay(t: f32) {
    sleep(Duration::from_secs_f32(t));
}
