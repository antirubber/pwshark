use clap::Parser;
use cli::{Args, ModeArg};
use gen::{generate_memorable, generate_random, MemorableConfig, RandomConfig};

pub mod cli;
pub mod gen;
pub mod tui;

fn main() {
    let args = Args::parse();

    if args.stdout {
        run_stdout(&args);
    } else {
        tui::run();
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
