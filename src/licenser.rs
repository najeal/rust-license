use super::{config, errors::LicenseError};

use regex::Regex;
use std::{
    cmp::Ordering,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    os::unix::fs::FileExt,
};

pub struct Licenser {
    file_paths: Vec<String>,
    cfg: config::Config,
    license_reg: Regex,
    header: String,
}

impl Licenser {
    pub fn new(file_pahs: Vec<String>, cfg: config::Config) -> Licenser {
        let header = add_carriage_return(cfg.headers.clone());
        let license_reg = generate_header_regex(&cfg.headers.clone());
        return Licenser {
            cfg: cfg,
            file_paths: file_pahs,
            license_reg: license_reg,
            header: header,
        };
    }

    pub fn check_files_license(&self) -> Result<Vec<&String>, LicenseError> {
        let file_toread_len = replace_header_placeholder(&self.header).len();
        let mut missing_license = vec![];
        for file_path in self.file_paths.iter() {
            let mut buffer = vec![0; file_toread_len];
            let file = File::open(file_path)?;
            match file.read_exact_at(&mut buffer, 0) {
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    missing_license.push(file_path);
                    continue;
                }
                Err(e) => return Err(LicenseError::from(e)),
                _ => (),
            }
            let file_content = String::from_utf8(buffer).unwrap();
            if !self.license_reg.is_match(&file_content) {
                missing_license.push(file_path);
            }
        }
        Ok(missing_license)
    }

    pub fn remove_files_license(&self) -> Result<bool, LicenseError> {
        let file_toread_len = replace_header_placeholder(&self.header).len();
        let mut to_remove = vec![];
        for file_path in self.file_paths.iter() {
            let mut buffer = vec![0; file_toread_len];
            let file = File::open(file_path)?;
            match file.read_exact_at(&mut buffer, 0) {
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    continue;
                }
                Err(e) => return Err(LicenseError::from(e)),
                _ => (),
            }
            let file_content = String::from_utf8(buffer).unwrap();
            if self.license_reg.is_match(&file_content) {
                to_remove.push(file_path);
            }
        }
        for remove in to_remove {
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(remove)
                .unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let new_content = remove_header(&self.license_reg, &content);
            file.set_len(0)?;
            file.write_all_at(new_content.as_bytes(), 0)?;
        }
        Ok(true)
    }

    pub fn apply_files_license(&self) -> Result<bool, LicenseError> {
        let file_toread_len = replace_header_placeholder(&self.header).len();
        let mut to_add = vec![];
        for file_path in self.file_paths.iter() {
            let mut buffer = vec![0; file_toread_len];
            let file = File::open(file_path)?;
            match file.read_exact_at(&mut buffer, 0) {
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    to_add.push(file_path);
                    continue;
                }
                Err(e) => return Err(LicenseError::from(e)),
                _ => (),
            }
            let file_content = String::from_utf8(buffer).unwrap();
            if !self.license_reg.is_match(&file_content) {
                to_add.push(file_path);
            }
        }
        for path in to_add {
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .open(path)
                .unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            let mut new_content = self.header.clone();
            new_content.push_str(&content);
            file.set_len(0)?;
            file.write_all_at(new_content.as_bytes(), 0)?;
        }
        Ok(true)
    }
}

/// replace_header_placeholder replaces {{YEAR}} by as 4 digits year
fn replace_header_placeholder(header: &str) -> String {
    return header.replace("{{YEAR}}", "2000");
}
/// reg_header generate a regex from the header
fn generate_header_regex(header: &str) -> Regex {
    let escaped_str = regex::escape(&header.replace("{{YEAR}}", "\\d{4}"));
    let reg_str = format!("^{}(?:\n|\r\n)", escaped_str);
    Regex::new(&reg_str).unwrap()
}

fn remove_header(reg: &Regex, content: &String) -> String {
    let new_content = reg.replace_all(content, "");
    return new_content.to_string();
}

fn add_carriage_return(mut input: String) -> String {
    if input.chars().last().unwrap() == '\n' as char {
        return input;
    }
    input.push_str("\n");
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use std::{fs, io};
    use tempdir::TempDir;

    const expected_headers: &str = "// Copyright (c) Company 2024";

    #[test]
    fn test_add_carriage_return() {
        assert_eq!(
            add_carriage_return(String::from("the string")),
            "the string\n"
        );
        assert_eq!(
            add_carriage_return(String::from("the string\n")),
            "the string\n"
        );
    }

    #[test]
    fn test_generate_header_regex() {
        let reg = generate_header_regex(expected_headers);
        assert_eq!(false, reg.is_match("// Copyright (c) Company 2024"));
        assert_eq!(true, reg.is_match("// Copyright (c) Company 2024\r\n"));
        assert_eq!(true, reg.is_match("// Copyright (c) Company 2024\n"));
        assert_eq!(
            true,
            reg.is_match("// Copyright (c) Company 2024\nfile content")
        );
    }

    #[test]
    fn test_remove_header() {
        let reg = generate_header_regex(expected_headers);
        assert_eq!(
            "file content",
            remove_header(
                &reg,
                &String::from("// Copyright (c) Company 2024\nfile content")
            )
        );
    }

    #[test]
    fn test_licenser_check_files_license() {
        let tmp_dir = TempDir::new("rust_license").unwrap();
        let tmp_no_header_1 = format!("{}/no_header_1.txt", tmp_dir.path().to_str().unwrap());
        let tmp_with_header_1 = format!("{}/with_header_1.txt", tmp_dir.path().to_str().unwrap());
        fs::copy("src/test_files/no_header_1.txt", tmp_no_header_1.clone()).unwrap();
        fs::copy(
            "src/test_files/with_header_1.txt",
            tmp_with_header_1.clone(),
        )
        .unwrap();

        let file_paths = vec![tmp_with_header_1.clone(), tmp_no_header_1.clone()];
        let licenser = Licenser::new(
            file_paths,
            config::Config {
                headers: expected_headers.to_string(),
            },
        );
        let list = licenser.check_files_license().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(**list.get(0).unwrap(), tmp_no_header_1);
    }

    #[test]
    fn test_licenser_remove_files_license() {
        let tmp_dir = TempDir::new("rust_license").unwrap();
        let tmp_no_header_1 = format!("{}/no_header_1.txt", tmp_dir.path().to_str().unwrap());
        let tmp_with_header_1 = format!("{}/with_header_1.txt", tmp_dir.path().to_str().unwrap());
        fs::copy("src/test_files/no_header_1.txt", tmp_no_header_1.clone()).unwrap();
        fs::copy(
            "src/test_files/with_header_1.txt",
            tmp_with_header_1.clone(),
        )
        .unwrap();

        let file_paths = vec![tmp_with_header_1.clone(), tmp_no_header_1.clone()];
        let licenser = Licenser::new(
            file_paths,
            config::Config {
                headers: expected_headers.to_string(),
            },
        );
        licenser.remove_files_license().unwrap();
        let mut fs = File::open(tmp_with_header_1).unwrap();
        let mut buf = vec![];
        fs.read_to_end(&mut buf).unwrap();
        let res = String::from_utf8(buf).unwrap();
        assert_eq!(res, "file content");
    }

    #[test]
    fn test_licenser_apply_files_license() {
        let tmp_dir = TempDir::new("rust_license").unwrap();
        let tmp_no_header_1 = format!("{}/no_header_1.txt", tmp_dir.path().to_str().unwrap());
        let tmp_with_header_1 = format!("{}/with_header_1.txt", tmp_dir.path().to_str().unwrap());
        fs::copy("src/test_files/no_header_1.txt", tmp_no_header_1.clone()).unwrap();
        fs::copy(
            "src/test_files/with_header_1.txt",
            tmp_with_header_1.clone(),
        )
        .unwrap();

        let file_paths = vec![tmp_with_header_1.clone(), tmp_no_header_1.clone()];
        let licenser = Licenser::new(
            file_paths,
            config::Config {
                headers: expected_headers.to_string(),
            },
        );
        licenser.apply_files_license().unwrap();
        let mut fs = File::open(tmp_no_header_1).unwrap();
        let mut buf = vec![];
        fs.read_to_end(&mut buf).unwrap();
        let res = String::from_utf8(buf).unwrap();
        assert_eq!(res, "// Copyright (c) Company 2024\nmy content");
    }
}
