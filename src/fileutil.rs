use std::fs;
use std::io::Read;
use std::path::Path;

pub fn get_file_text(path: &Path) -> String {
    let mut file: fs::File = fs::File::open(path).expect("Unreacheable");
    let mut buf: String = String::from("");
    file.read_to_string(&mut buf);
    return buf;
}