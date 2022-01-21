use std::{fs, io};
use std::path::Path;
use std::io::BufRead;

fn main() {
    visit_files_rec("./");
}

fn find_todos(path: &Path) {
    let file = fs::File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut ret: Vec<&str> = vec![];
    let mut first = true;

    for line in lines {
        if let Ok(line) = line {
            let mut aux = line.split("TODO: ");
            aux.next();
            if let Some(text) = aux.next() {
                if first {
                    println!("{}", path.display());
                    first = false;
                }

                println!("\t{}", text);
            }
        }
    }
}

fn visit_files_rec(path: &str) {
    for path in fs::read_dir(path).unwrap() {
        let file_path = path.unwrap().path();
        if file_path.is_dir() {
            visit_files_rec(file_path.to_str().unwrap());
        } else {
            find_todos(&file_path);
        }
    }
}