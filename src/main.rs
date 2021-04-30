extern crate clap;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg, SubCommand};

#[path = "pretty-make/mod.rs"]
mod pretty_make;

#[path = "pretty-template/mod.rs"]
mod pretty_template;

// Source: https://stackoverflow.com/a/27841363
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let matches = App::new("Pretty Make")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommands(vec![
            SubCommand::with_name("pretty-help")
                .about("Parse a Makefile to produce a pretty help.")
                .arg(
                    Arg::with_name("makefile")
                        .help("Makefile to be pretty")
                        .required(true),
                ),
            SubCommand::with_name("template")
                .about("Update or create a Makefile using the given template.")
                .arg(
                    Arg::with_name("file")
                        .help("File to udpated or created with a template")
                        .required(true),
                )
                .arg(
                    Arg::with_name("url")
                        .help("Url of template to use for update or create")
                        .required(true),
                ),
        ])
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("pretty-help") {
        if let Some(makefile) = matches.value_of("makefile") {
            pretty_make::main::print_help(makefile);
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("template") {
        match (matches.value_of("file"), matches.value_of("url")) {
            (Some(file), Some(url)) => {
                pretty_template::main::update_or_create_using_template(file, url)
            }
            _ => (),
        }
    }
}
