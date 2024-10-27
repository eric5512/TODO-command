use regex::Regex;

pub fn get_file_regex(ext: &str) -> Option<Regex> {
    match ext {
        "c" | "cpp" | "h" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        "js" | "ts" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        "ml" | "mli" => Some(Regex::new(r"\(\*\*?([^*]|([*][^)]))*TODO:([^*]|([*][^)]))*\**\*\)").unwrap()),
        "py" => Some(Regex::new(r#"(#.*TODO:*.)|("""([^"])*TODO:([^"])*""")"#).unwrap()),
        "rs" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        "java" => Some(Regex::new(r"(//.*TODO:.*)|(/[*]([^*]|([*][^/]))*TODO:([^*]|([*][^/]))*[*]+/)").unwrap()),
        _ => None
    }
}

pub fn get_matches(text: &str, regex: Regex) -> Vec<(u64, u64, &str)> {
    let mut result: Vec<(u64, u64, &str)> = Vec::new();
    let mut line_start_indices: Vec<usize> = vec![0]; // Track start index of each line

    // Calculate the start index of each line in the text
    for (i, ch) in text.chars().enumerate() {
        if ch == '\n' {
            line_start_indices.push(i + 1);
        }
    }

    // Find all matches
    for mat in regex.find_iter(text) {
        let match_start: usize = mat.start();

        // Calculate the line number
        let line_number: u64 = line_start_indices
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, &start_idx)| match_start >= start_idx)
            .map(|(line_idx, _)| line_idx as u64 + 1)
            .unwrap_or(1);

        // Calculate the column number (relative to the line start)
        let line_start: usize = line_start_indices[line_number as usize - 1];
        let column: u64 = (match_start - line_start) as u64 + 1;

        // Push the match with its line, column, and matched text
        if let Some((line, _)) = text[match_start..].split_once("\n") {
            result.push((line_number, column, line));
        } else {
            result.push((line_number, column, &text[match_start..]));
        }

    }

    result
}