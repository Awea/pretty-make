extern crate clap;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::App;

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
        .arg("<makefile> 'Makefile to be pretty'")
        .get_matches();

    let makefile = matches.value_of("makefile").unwrap();
    pretty_make::main::print_help(makefile);
}
