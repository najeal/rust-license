use regex::Regex;
use std::{cmp::Ordering, fs::File, os::unix::fs::FileExt};

/// fake_header generate replace {{YEAR}} by as 4 digits year
fn fake_header(header: &str) -> String {
    return header.replace("{{YEAR}}", "2000");
}

/// reg_header generate a regex from the header
fn reg_header(header: &str) -> Regex {
    let escaped_str = regex::escape(&header.replace("{{YEAR}}", "\\d{4}"));
    let reg_str = format!("^{}", escaped_str);
    Regex::new(&reg_str).unwrap()
}

/// contains_header checks if the begin of a file contains the expected header
fn contains_header(file: File, header: &str) -> bool {
    let len = fake_header(header).len();
    let mut buffer = vec![0; len];
    file.read_exact_at(&mut buffer, 0).unwrap();
    let file_content = String::from_utf8(buffer).unwrap();
    reg_header(header).is_match(&file_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    const expected_headers: &str = "// Copyright (c) Company 2024";

    #[test]
    fn test_contains_header() {
        let file = File::open("src/check_test.txt").unwrap();
        assert_eq!(contains_header(file, expected_headers), true)
    }
}
