use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;

mod regexlang;
mod fileutil;

fn main() {
    visit_files_rec("./");
}

fn print_todos(path: &Path, regex: Regex) {
    let text = fileutil::get_file_text(path);

    let matches = regexlang::get_matches(text, regex);

    if matches.len() > 0 {
        print!("Found {} TODOS in file {}", matches.len(), path.display());
        
        for ma in matches {
            let (line, col, text) = ma;
            print!("\t{}:{}:{} {}", path.display(), line, col, text);

        }
    }
}

fn visit_files_rec(path: &str) {
    for path in fs::read_dir(path).unwrap() {
        let file_path = path.unwrap().path();
        if let Some(regex) = file_path.extension().and_then(OsStr::to_str).and_then(regexlang::get_file_regex) {
            print_todos(&file_path, regex);
        } else if file_path.is_dir() {
            visit_files_rec(file_path.to_str().unwrap());
        }
    }
}