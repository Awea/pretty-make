extern crate pest;
#[macro_use]
extern crate pest_derive;

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

    for record in file {
        match record.as_rule() {
            Rule::target_with_help => {
                let name = record
                    .clone()
                    .into_inner()
                    .find(|x| x.as_rule() == Rule::target_name)
                    .unwrap();

                let help_messages = record
                    .clone()
                    .into_inner()
                    .find(|x| x.as_rule() == Rule::help_message)
                    .map(|x| String::from(x.as_str()))
                    .unwrap();

                let target_with_help_messages = TargetWithHelpMessage {
                    name: String::from(name.as_str()),
                    help_messages: vec![help_messages],
                };
                targets.targets.push(target_with_help_messages)
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:?}", targets);
    // println!("{:?}", target_with_help_messages.help_messages);
}
