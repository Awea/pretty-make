extern crate clap;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, AppSettings, Arg, SubCommand};

#[path = "pretty-make/mod.rs"]
mod pretty_make;

// Source: https://stackoverflow.com/a/27841363
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    // Default subcommand, like bat: `pretty-make Makefile` runs `pretty-help`.
    let mut args: Vec<_> = std::env::args_os().collect();
    if let Some(first) = args.get(1).and_then(|arg| arg.to_str()) {
        if !first.starts_with('-') && first != "pretty-help" && first != "help" {
            args.insert(1, "pretty-help".into());
        }
    }

    let matches = App::new("Pretty Make")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            SubCommand::with_name("pretty-help")
                .about("Parse a Makefile to produce a pretty help.")
                .arg(
                    Arg::with_name("makefile")
                        .help("Makefile to be pretty")
                        .required(true),
                ),
        ])
        .get_matches_from(args);

    if let Some(ref matches) = matches.subcommand_matches("pretty-help") {
        if let Some(makefile) = matches.value_of("makefile") {
            pretty_make::main::print_help(makefile);
        }
    }
}
