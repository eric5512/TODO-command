use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use regex::Regex;
use clap::Parser;

mod regexlang;
mod fileutil;


/// Simple program to retrieve the TODOS in a project
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Look recursively into the nested folders
    #[arg(short, long)]
    recursive: bool,

    /// Specify the extension of the files to look into
    #[arg(short, long)]
    extensions: Option<Vec<String>>
}

fn main() {
    let args: Args = Args::parse();
    visit_files_rec("./", &args);
}

fn print_todos(path: &Path, regex: Regex) {
    let text: String = fileutil::get_file_text(path).expect(&format!("Warningn: Couldn't read the file {}", path.as_os_str().to_str().unwrap()));

    let matches: Vec<(u64, u64, &str)> = regexlang::get_matches(&text, regex);

    if matches.len() > 0 {
        print!("Found {} TODOS in file {}\n", matches.len(), path.display());
        
        for ma in matches {
            let (line, col, text) = ma;
            print!("\t{}:{}:{} {}\n", path.display(), line, col, text);
        }
    }
}

fn visit_files_rec(path: &str, args: &Args) {
    for path in fs::read_dir(path).unwrap() {
        let file_path: PathBuf = path.unwrap().path();
        let extension: Option<&str> = file_path.extension().and_then(OsStr::to_str);
        if let Some(regex) = extension.and_then(regexlang::get_file_regex) {
            if let Some(exts) = &args.extensions {
                if exts.contains(&extension.unwrap().to_string()) {
                    print_todos(&file_path, regex);
                }
            } else {
                print_todos(&file_path, regex);
            }
        } else if file_path.is_dir() && args.recursive {
            visit_files_rec(file_path.to_str().unwrap(), args);
        }
    }
}