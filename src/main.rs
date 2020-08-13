extern crate colorful;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use colorful::Colorful;
use colorful::RGB;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "makefile.pest"]
struct MakefileParser;

#[derive(Debug)]
struct Targets {
    targets: Vec<TargetWithHelpMessage>,
}

#[derive(Debug)]
struct TargetWithHelpMessage {
    name: String,
    help_messages: Vec<String>,
}

fn main() {
    let unparsed_file = fs::read_to_string("tests/fixtures/Makefile").expect("cannot read file");
    let file =
        MakefileParser::parse(Rule::makefile, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));
    let mut targets = Targets {
        targets: Vec::new(),
    };

    // println!("{:?}", file);

    for record in file {
        match record.as_rule() {
            Rule::target_with_help => {
                // println!("{:?}", record);

                let name = record
                    .clone()
                    .into_inner()
                    .find(|x| x.as_rule() == Rule::target_name)
                    .unwrap();

                let help_messages = record
                    .clone()
                    .into_inner()
                    .filter(|x| x.as_rule() == Rule::help_message)
                    .map(|x| String::from(x.as_str()))
                    .collect();

                let target_with_help_messages = TargetWithHelpMessage {
                    name: String::from(name.as_str()),
                    help_messages: help_messages,
                };
                targets.targets.push(target_with_help_messages)
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    // primary:
    // - #a6cc70
    //  - `RGB::new(166, 204, 112)`
    // - bold
    // title:
    // - #ffcc66
    //  - `RGB::new(255, 204, 102)`
    // link:
    // - #77a8d9
    //  - `RGB::new(119,168,217)`
    let primary = RGB::new(166, 204, 112);
    let title = RGB::new(255, 204, 102);
    let link = RGB::new(166, 204, 112);

    println!("{}", "my-project".color(primary).bold());
    println!("{} \n", "Project description using Make");
    println!("{}", "USAGE".color(title));
    println!("    {}\n", "make <SUBCOMMAND>");
    println!("{}", "SUBCOMMANDS".color(title));

    for target in targets.targets {
        println!(
            "{}    {:?}",
            target.name.color(primary).bold(),
            target.help_messages
        );
    }
}
