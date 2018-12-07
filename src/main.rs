mod cmd;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;

fn init_app<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(Arg::with_name("track_dir")
             .short("d")
             .long("track-dir")
             .help("A path to the Exercism track repo. If not set, defaults to the current directory.")
             .takes_value(true)
             .default_value("."))
        .subcommand(
            SubCommand::with_name("outdated")
                .about("List all outdated exercises on the current track"),
        ).subcommand(
            SubCommand::with_name("missing")
                .about("List all unimplemented exercises on the current track"),
        ).get_matches()
}

fn process_matches(matches: &ArgMatches<'_>) -> xtodo::Result<()> {
    let track_dir = Path::new(matches.value_of("track_dir").unwrap());

    match matches.subcommand() {
        ("missing", _) => cmd::list_missing_exercises(&track_dir),

        ("outdated", _) => cmd::list_outdated_exercises(&track_dir),

        ("", _) => {
            println!("No subcommand was used.\nUse 'xtodo help' to learn about the possible subcommands.");
            Ok(())
        }

        _ => unreachable!(),
    }
}

fn main() -> xtodo::Result<()> {
    let matches = init_app();

    process_matches(&matches)
}
