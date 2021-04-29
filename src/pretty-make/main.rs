use colored::*;
use colors_transform::{Color, Rgb};
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

const INDENT_WIDTH: usize = 4;
const DEFAULT_COLOR_TITLE: &str = "#a6cc70";
const DEFAULT_COLOR_SUBTITLE: &str = "#ffcc66";
const DEFAULT_COLOR_LINK: &str = "#77a8d9";

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

// Print a pretty help using the given path to a **Makefile** with the following syntax:
// ```Makefile
// #@name Project name
// #@description Project description. (optional)
// #@color-title #70c3cc (optional)
// #@color-subtitle #c970cc (optional)
// #@color-link #0314fc (optional)

// # This is a comment and it won't appear in the `make help`.

// ## Build site for production use
// .PHONY: build
// build: deps
//   @echo "Building front-end"
//   @rm -rf site/*
//   @NODE_ENV=production $(WEBPACK) --config webpack/prod.js
//   @echo "Front-end built!"

// .DEFAULT_GOAL := serve
// ## Serve:
// ## - Site at http://localhost:3000 with hot reloading
// ## - API at http://localhost:3010
// ## - phpMyAdmin at http://localhost:3011
// .PHONY: serve
// serve: api deps
//   @$(WEBPACK_SERVER) --inline --progress --config webpack/dev.js
// ```
pub fn print_help(makefile: &str) {
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
    let mut color_link: Rgb = hex_to_rgb(DEFAULT_COLOR_LINK.to_string());

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
            Rule::color_link => {
                color_link = hex_to_rgb(get_text(&record, Rule::color));
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
        println!("{}", project_description);
    }

    // Add a new line before displaying USAGE
    // - @awea 20210407
    println!("");
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