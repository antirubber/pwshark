use clap::Parser;
use pwshark::cli::{Args, Command};

#[test]
fn update_subcommand_parses() {
    let args = Args::try_parse_from(["pwshark", "update"]).unwrap();
    assert!(matches!(args.command, Some(Command::Update)));
}

#[test]
fn no_subcommand_still_parses_flags() {
    let args = Args::try_parse_from(["pwshark", "--stdout"]).unwrap();
    assert!(args.command.is_none());
    assert!(args.stdout);
}
