extern crate clap;
extern crate colored;
extern crate colors_transform;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::App;
use colored::*;
use colors_transform::{Color, Rgb};
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "makefile.pest"]
struct MakefileParser;

#[derive(Debug)]
struct Targets {
    targets: Vec<TargetWithHelpMessage>,
}

#[derive(Debug, Clone)]
struct TargetWithHelpMessage {
    target_name: String,
    help_messages: Vec<String>,
}

const INDENT_WIDTH: usize = 4;
const DEFAULT_DESCRIPTION: &str = "This project uses Make for commands.";
const DEFAULT_COLOR_TITLE: &str = "#a6cc70";
const DEFAULT_COLOR_SUBTITLE: &str = "#ffcc66";

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
    let unparsed_file = fs::read_to_string(makefile).expect("cannot read file");
    let file =
        MakefileParser::parse(Rule::makefile, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));
    let mut targets = Targets {
        targets: Vec::new(),
    };
    let mut project_name: String = "".to_string();
    let mut project_description: String = "".to_string();
    let mut color_title: Rgb = hex_to_rgb(DEFAULT_COLOR_TITLE.to_string());
    let mut color_subtitle: Rgb = hex_to_rgb(DEFAULT_COLOR_SUBTITLE.to_string());

    // To customize color link I need to rewrite how I colorize links inside the following
    // `for` loop because I need to know if the color is customized before I can actually
    // customize it - @awea 20200913
    let color_link: Rgb = hex_to_rgb("#77a8d9".to_string());

    for record in file {
        match record.as_rule() {
            Rule::target_with_help => {
                let target_name = get_text(&record, Rule::target_name);
                let help_messages = record
                    .clone()
                    .into_inner()
                    .filter(|x| x.as_rule() == Rule::text)
                    .map(|x| {
                        let links: Vec<String> = x
                            .clone()
                            .into_inner()
                            .filter(|x| x.as_rule() == Rule::link)
                            .map(|x| String::from(x.as_str()))
                            .collect();

                        let mut result = String::from(x.as_str());

                        for link in links.iter() {
                            let colored_link =
                                format!("{}", color_text(link.to_string(), color_link));

                            result = result.replace(link, &colored_link);
                        }

                        result
                    })
                    .collect();

                let target_with_help_messages = TargetWithHelpMessage {
                    target_name,
                    help_messages,
                };
                targets.targets.push(target_with_help_messages)
            }
            Rule::name => {
                project_name = get_text(&record, Rule::text);
            }
            Rule::description => {
                project_description = get_text(&record, Rule::text);
            }
            Rule::color_title => {
                color_title = hex_to_rgb(get_text(&record, Rule::color));
            }
            Rule::color_subtitle => {
                color_subtitle = hex_to_rgb(get_text(&record, Rule::color));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let help_message_offset = help_message_offset(&targets);

    // Help output starts here, colors used are the followings:
    // - Primary
    //  - RGB: 166, 204, 112
    //  - bold
    // - title
    //  - RGB: 255, 204, 102
    // - link
    //  - RGB: 119, 168, 217
    println!("{}", color_text(project_name, color_title).bold());
    if project_description.len() > 0 {
        println!("{} {} \n", project_description, DEFAULT_DESCRIPTION);
    } else {
        println!("{} \n", DEFAULT_DESCRIPTION);
    }
    println!("{}", color_text("USAGE".to_string(), color_subtitle));
    println!("    {}\n", "make <SUBCOMMAND>");
    println!("{}", color_text("SUBCOMMANDS".to_string(), color_subtitle));

    for target in targets.targets {
        print!("{: <1$}", "", INDENT_WIDTH);
        print!(
            "{target_name: <col$}",
            target_name = color_text(target.target_name, color_title).bold(),
            col = help_message_offset
        );

        let mut i = 0;
        for help_message in target.help_messages {
            if i > 0 {
                print!("{: <1$}", "", INDENT_WIDTH);
                print!("{: <1$}", "", help_message_offset);
            }
            println!("{}", help_message);

            i = i + 1;
        }
    }
}

fn hex_to_rgb(hex: String) -> Rgb {
    Rgb::from_hex_str(&hex).unwrap()
}

fn color_text(text: String, color: Rgb) -> ColoredString {
    text.truecolor(
        color.get_red() as u8,
        color.get_green() as u8,
        color.get_blue() as u8,
    )
}

fn get_text(record: &Pair<Rule>, rule_type: Rule) -> String {
    let line = record
        .clone()
        .into_inner()
        .find(|x| x.as_rule() == rule_type)
        .unwrap();

    String::from(line.as_str())
}

fn help_message_offset(targets: &Targets) -> usize {
    let longest_target_name =
        targets
            .targets
            .iter()
            .fold(targets.targets[0].clone(), |acc, item| {
                if item.target_name.len() > acc.target_name.len() {
                    item.clone()
                } else {
                    acc
                }
            });

    longest_target_name.target_name.len() + INDENT_WIDTH
}
