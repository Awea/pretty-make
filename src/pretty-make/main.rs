use colored::*;
use colors_transform::{Color, Rgb};
use pest::iterators::Pair;
use pest::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const INDENT_WIDTH: usize = 4;
const DEFAULT_COLOR_TITLE: &str = "#a6cc70";
const DEFAULT_COLOR_SUBTITLE: &str = "#ffcc66";
const DEFAULT_COLOR_LINK: &str = "#77a8d9";

#[derive(Parser)]
#[grammar = "makefile.pest"]
struct MakefileParser;

#[derive(Debug, Clone)]
enum Help {
    Target(TargetHelp),
    Section(HelpSection),
}

#[derive(Debug)]
struct Targets {
    targets: Vec<Help>,
}

#[derive(Debug, Clone)]
struct TargetHelp {
    target_name: String,
    help_messages: Vec<String>,
}

#[derive(Debug, Clone)]
struct HelpSection {
    title: String,
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
    let makefile_path = Path::new(makefile);
    let mut targets = Targets {
        targets: Vec::new(),
    };
    let mut project_name: String = "".to_string();
    let mut project_description: String = "".to_string();
    let mut color_title: Rgb = hex_to_rgb(DEFAULT_COLOR_TITLE.to_string());
    let mut color_subtitle: Rgb = hex_to_rgb(DEFAULT_COLOR_SUBTITLE.to_string());
    let mut color_link: Rgb = hex_to_rgb(DEFAULT_COLOR_LINK.to_string());
    let mut includes = Vec::new();

    parse_makefile(
        makefile_path,
        &mut targets,
        &mut project_name,
        &mut project_description,
        &mut color_title,
        &mut color_subtitle,
        &mut color_link,
        &mut includes,
    );

    // Process includes after main file is parsed
    let include_dirs = get_include_dirs_from_makeflags();

    for include_path in includes {
        let base_dir = makefile_path.parent().unwrap_or(Path::new("."));
        let included_file_path = find_included_file(include_path.trim(), base_dir, &include_dirs);

        if let Some(included_file_path) = included_file_path {
            let mut included_targets_container = Targets {
                targets: Vec::new(),
            };
            let mut nested_includes = Vec::new();
            parse_makefile(
                &included_file_path,
                &mut included_targets_container,
                &mut project_name,
                &mut project_description,
                &mut color_title,
                &mut color_subtitle,
                &mut color_link,
                &mut nested_includes,
            );

            if !included_targets_container.targets.is_empty() {
                let file_path_str = included_file_path.to_string_lossy().to_string();
                let section_title = format!("Help for {}:", file_path_str);
                let help_section = HelpSection {
                    title: section_title,
                };
                targets.targets.push(Help::Section(help_section));
                targets.targets.extend(included_targets_container.targets);
            }

            // Process nested includes
            for nested_include_path in nested_includes {
                let nested_base_dir = included_file_path.parent().unwrap_or(Path::new("."));
                let nested_included_file_path =
                    find_included_file(nested_include_path.trim(), nested_base_dir, &include_dirs);

                if let Some(nested_included_file_path) = nested_included_file_path {
                    let mut nested_targets_container = Targets {
                        targets: Vec::new(),
                    };
                    let mut nested_nested_includes = Vec::new();
                    parse_makefile(
                        &nested_included_file_path,
                        &mut nested_targets_container,
                        &mut project_name,
                        &mut project_description,
                        &mut color_title,
                        &mut color_subtitle,
                        &mut color_link,
                        &mut nested_nested_includes,
                    );

                    if !nested_targets_container.targets.is_empty() {
                        let nested_file_path_str =
                            nested_included_file_path.to_string_lossy().to_string();
                        let nested_section_title = format!("Help for {}:", nested_file_path_str);
                        let nested_help_section = HelpSection {
                            title: nested_section_title,
                        };
                        targets.targets.push(Help::Section(nested_help_section));
                        targets.targets.extend(nested_targets_container.targets);
                    }
                }
            }
        }
    }

    if targets.targets.len() == 0 {
        println!("{}", "No help message found please read the readme: https://github.com/awea/pretty-make#readme".bold());

        process::exit(1);
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
        match target {
            Help::Target(target) => {
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
            Help::Section(target) => {
                println!("");
                print!("{: <1$}", "", INDENT_WIDTH / 2);
                println!("{}", color_text(target.title, color_subtitle));
                println!("");
            }
        }
    }
}

fn parse_makefile(
    makefile_path: &Path,
    targets: &mut Targets,
    project_name: &mut String,
    project_description: &mut String,
    color_title: &mut Rgb,
    color_subtitle: &mut Rgb,
    color_link: &mut Rgb,
    includes: &mut Vec<String>,
) {
    let unparsed_file = fs::read_to_string(makefile_path).expect("cannot read file");
    let file =
        MakefileParser::parse(Rule::makefile, &unparsed_file).unwrap_or_else(|e| panic!("{}", e));

    for record in file {
        match record.as_rule() {
            Rule::target_with_help => {
                let target_name = get_text(&record, Rule::target_name);
                let help_messages = record
                    .into_inner()
                    .filter(|x| x.as_rule() == Rule::text)
                    .map(|x| {
                        let mut result = String::from(x.as_str());

                        let links: Vec<String> = x
                            .into_inner()
                            .filter(|x| x.as_rule() == Rule::link)
                            .map(|x| String::from(x.as_str()))
                            .collect();

                        for link in links.iter() {
                            let colored_link =
                                format!("{}", color_text(link.to_string(), *color_link));

                            result = result.replace(link, &colored_link);
                        }

                        result
                    })
                    .collect();

                let target_with_help_messages = TargetHelp {
                    target_name,
                    help_messages,
                };
                targets
                    .targets
                    .push(Help::Target(target_with_help_messages))
            }
            Rule::help_section => {
                let title = get_text(&record, Rule::help_section_title);

                let help_section = HelpSection { title };
                targets.targets.push(Help::Section(help_section))
            }
            Rule::name => {
                if project_name.is_empty() {
                    *project_name = get_text(&record, Rule::text);
                }
            }
            Rule::description => {
                if project_description.is_empty() {
                    *project_description = get_text(&record, Rule::text);
                }
            }
            Rule::color_title => {
                *color_title = hex_to_rgb(get_text(&record, Rule::color));
            }
            Rule::color_subtitle => {
                *color_subtitle = hex_to_rgb(get_text(&record, Rule::color));
            }
            Rule::color_link => {
                *color_link = hex_to_rgb(get_text(&record, Rule::color));
            }
            Rule::include_statement => {
                let include_path = get_text(&record, Rule::include_path);
                includes.push(include_path);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}

fn get_include_dirs_from_makeflags() -> Vec<String> {
    let mut include_dirs = Vec::new();

    if let Ok(makeflags) = env::var("MAKEFLAGS") {
        let parts: Vec<&str> = makeflags.split_whitespace().collect();
        let mut i = 0;

        while i < parts.len() {
            if parts[i] == "-I" && i + 1 < parts.len() {
                include_dirs.push(parts[i + 1].to_string());
                i += 2;
            } else if parts[i] == "I" && i + 1 < parts.len() {
                // Handle I path format (without dash)
                include_dirs.push(parts[i + 1].to_string());
                i += 2;
            } else if parts[i].starts_with("-I") {
                // Handle -Ipath format
                let path = &parts[i][2..];
                if !path.is_empty() {
                    include_dirs.push(path.to_string());
                }
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    include_dirs
}

fn find_included_file(
    include_path: &str,
    base_dir: &Path,
    include_dirs: &[String],
) -> Option<std::path::PathBuf> {
    // First try relative to the current makefile's directory
    let local_path = base_dir.join(include_path);
    if local_path.exists() {
        return Some(local_path);
    }

    // Then try each directory from MAKEFLAGS -I
    for include_dir in include_dirs {
        let search_path = Path::new(include_dir).join(include_path);
        if search_path.exists() {
            return Some(search_path);
        }

        // Also try relative to base_dir + include_dir
        let relative_search_path = base_dir.join(include_dir).join(include_path);
        if relative_search_path.exists() {
            return Some(relative_search_path);
        }
    }

    None
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
    let longest_target_name = targets.targets.iter().fold(0, |acc, item| match item {
        Help::Target(target) => {
            if target.target_name.len() > acc {
                target.target_name.len()
            } else {
                acc
            }
        }
        _ => acc,
    });

    longest_target_name + INDENT_WIDTH
}
