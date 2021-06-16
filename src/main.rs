extern crate clap;
extern crate clap_generate;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg};
use clap_generate::{generate, generators::Bash};
use std::io;

#[path = "pretty-make/mod.rs"]
mod pretty_make;

// Source: https://stackoverflow.com/a/27841363
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let mut app = App::new("Pretty Make")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommand(
            App::new("pretty-help")
                .about("Parse a Makefile to produce a pretty help.")
                .arg(
                    Arg::new("makefile")
                        .about("Makefile to be pretty")
                        .required(true),
                ),
        );

    // Auto-completion is a feature in clap 3 but it doesn't work yet
    // - @awea 20210617
    // Issues: https://github.com/clap-rs/clap/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+completion
    // generate::<Bash, _>(&mut app, "Pretty Make", &mut io::stdout());

    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("pretty-help") {
        if let Some(makefile) = matches.value_of("makefile") {
            pretty_make::main::print_help(makefile);
        }
    }
}
