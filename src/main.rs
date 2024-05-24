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
            let s = licenser::Licenser::new(cmd.files, cfg);
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
