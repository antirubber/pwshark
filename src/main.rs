use clap::Parser;
use cli::{Args, Command, ModeArg};
use gen::{generate_memorable, generate_random, MemorableConfig, RandomConfig};

pub mod cli;
pub mod gen;
pub mod tui;

const INSTALL_URL: &str =
    "https://raw.githubusercontent.com/antirubber/pwshark/main/install.sh";

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Update) => run_update(),
        None => {
            if args.stdout {
                run_stdout(&args);
            } else {
                tui::run();
            }
        }
    }
}

fn run_update() {
    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("curl -fsSL {INSTALL_URL} | bash"))
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => std::process::exit(s.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("pwshark update: failed to launch installer: {e}");
            std::process::exit(1);
        }
    }
}

fn run_stdout(args: &Args) {
    let mut rng = rand::rng();
    let password = match args.mode {
        ModeArg::Random => generate_random(
            &mut rng,
            &RandomConfig {
                length: args.length,
                uppercase: args.get_uppercase(),
                lowercase: args.get_lowercase(),
                numbers: args.get_numbers(),
                symbols: args.get_symbols(),
            },
        ),
        ModeArg::Memorable => generate_memorable(
            &mut rng,
            &MemorableConfig {
                word_count: args.words,
                separator: args.separator.clone(),
                capitalize: args.get_capitalize(),
                add_numbers: args.get_add_numbers(),
                truncate: args.get_truncate(),
            },
        ),
    };
    println!("{}", password.as_str());
}
