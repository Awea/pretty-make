extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "makefile.pest"]
pub struct MakefileParser;

fn main() {
    let unparsed_file = fs::read_to_string("tests/fixtures/Makefile").expect("cannot read file");
    let file = MakefileParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::target_with_help => {
                for field in record.into_inner() {
                    println!("{:?}", field);
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
