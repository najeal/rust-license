use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub headers: String,
}

pub fn load(path: &str) -> io::Result<Config> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(parse(&content.as_str()))
}

fn parse(c: &str) -> Config {
    let cfg: Config = serde_yaml::from_str(c).expect("cannot parse the config");
    return cfg;
}

#[cfg(test)]
mod tests {
    use super::*;
    const expected_headers: &str = "// Copyright (c) Company 2024";

    #[test]
    fn test_parse() {
        let yaml = include_str!("config_test.yaml");
        //let yaml = "headers: |
        // Copyright (c) Company 2024";
        let res = parse(yaml);
        assert_eq!(res.headers, expected_headers);
    }

    #[test]
    fn test_load() {
        let cfg = load("src/config_test.yaml").expect("failed to load config");
        assert_eq!(cfg.headers, expected_headers);
    }
}
