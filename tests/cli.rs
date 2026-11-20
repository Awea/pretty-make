use std::path::PathBuf;
use std::process::{Command, Output};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/Makefile")
}

fn run(args: &[&str]) -> Output {
    run_in(env!("CARGO_MANIFEST_DIR"), args)
}

fn run_in(dir: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pretty-make"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run pretty-make")
}

#[test]
fn pretty_help_subcommand_prints_help() {
    let makefile = fixture();
    let output = run(&["pretty-help", makefile.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn makefile_without_subcommand_prints_same_help() {
    let makefile = fixture();
    let expected = run(&["pretty-help", makefile.to_str().unwrap()]);
    let output = run(&[makefile.to_str().unwrap()]);

    assert!(expected.status.success());
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&expected.stdout)
    );
}

#[test]
fn no_args_prints_clap_help() {
    let output = run(&[]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!output.status.success());
    assert!(text.contains("USAGE"), "output: {}", text);
    assert!(text.contains("pretty-help"), "output: {}", text);
}

#[test]
fn relative_makefile_named_like_a_subcommand_prints_help() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
    let expected = run_in(dir, &["pretty-help", "help.mk"]);
    let output = run_in(dir, &["help.mk"]);

    assert!(expected.status.success());
    assert!(!expected.stdout.is_empty());
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&expected.stdout)
    );
}

#[test]
fn version_flag_prints_version() {
    let output = run(&["--version"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Pretty Make"));
}

#[test]
fn help_subcommand_prints_clap_help() {
    let output = run(&["help"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("USAGE"));
}
