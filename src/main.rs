extern crate clap;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg, SubCommand};

#[path = "pretty-make/mod.rs"]
mod pretty_make;

// Source: https://stackoverflow.com/a/27841363
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new("Pretty Make")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("makefile")
                .help("Makefile to be pretty")
                .index(1)
                .default_value("Makefile"),
        )
        .subcommands(vec![
            SubCommand::with_name("pretty-help")
                .about("Parse a Makefile to produce a pretty help.")
                .arg(
                    Arg::with_name("makefile")
                        .help("Makefile to be pretty")
                        .required(true),
                ),
        ])
        .get_matches();

    let default_makefile = matches.value_of("makefile").unwrap();

    if let Some(ref sub_matches) = matches.subcommand_matches("pretty-help") {
        let makefile = sub_matches.value_of("makefile").unwrap_or(default_makefile);
        pretty_make::main::print_help(makefile);
    } else {
        eprintln!("Error: to produce a pretty help please use `pretty-make pretty-help {}`", default_makefile);
        std::process::exit(1);
    }
}
