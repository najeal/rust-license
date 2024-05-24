mod check;
mod config;
mod errors;
mod licenser;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "rust-license")]
#[command(version = "0.1")]
#[command(about= "program to manage rust-licenses", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: RootSubCommand,
}

#[derive(Subcommand, Debug)]
enum RootSubCommand {
    LicenseHeader(LicenseHeader),
}

#[derive(Parser, Debug)]
struct LicenseHeader {
    #[arg(long)]
    config: String,
    #[arg(short, long)]
    apply: bool,
    #[arg(short, long)]
    remove: bool,
    #[arg(short, long)]
    check: bool,
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        RootSubCommand::LicenseHeader(cmd) => {
            let cfg = config::load(&cmd.config).unwrap();
            if !(cmd.apply ^ cmd.remove ^ cmd.check) {
                panic!("apply - remove - check flags should be used independently")
            }

            let s = licenser::Licenser::new(cleanup_file_list(&cmd.files), cfg);
            if cmd.apply {
                let _ = s.apply_files_license().unwrap();
            }
            if cmd.remove {
                let _ = s.remove_files_license().unwrap();
            }
            if cmd.check {
                let no_license_list = s.check_files_license().unwrap();
                for elem in no_license_list.iter() {
                    println!("{}", elem);
                }
            }
        }
        _ => (),
    }
}

fn cleanup_file_list(paths: &Vec<String>) -> Vec<String>{
    let mut out: Vec<String> = vec![];
    for path in paths {
        let splitted: Vec<&str> = path.split('\n').collect();
        for item in splitted {
            if item == "" {
                continue
            }
            out.push(item.to_string())
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::cleanup_file_list;

    #[test]
    fn test_cleanup_file_list() {
        let input: Vec<String> = vec![String::from("src/licenser.rs\nsrc/main.rs\n"), String::from("src/errors.rs")];
        let output = cleanup_file_list(&input);
        assert_eq!(output, vec![String::from("src/licenser.rs"), String::from("src/main.rs"), String::from("src/errors.rs")]);
    }
}