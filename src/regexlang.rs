use regex::Regex;

pub fn get_file_regex(ext: &str) -> Option<Regex> {
    match ext {
        "c" | "cpp" | "h" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        "js" | "ts" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        "ml" | "mli" => Some(Regex::new(r"\(\*\*?([^*]|([*][^)]))*TODO:([^*]|([*][^)]))*\**\*\)").unwrap()),
        "py" => Some(Regex::new(r#"(#.*TODO:*.)|("""([^"])*TODO:([^"])*""")"#).unwrap()),
        "rs" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        _ => None
    }
}

pub fn get_matches(text: &str, regex: Regex) -> Vec<(u64, u64, &str)> {
    return vec![];
}