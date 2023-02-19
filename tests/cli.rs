use std::process::Command;

use assert_cmd::prelude::*;

const PROGRAM_NAME: &'static str = "echo_sab";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn test_echo_with_options(input: &str, expected_output: &str, options: &[&str]) -> TestResult {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    let output = cmd.args(options).arg(input).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    assert_eq!(expected_output, stdout);
    Ok(())
}

#[test]
fn test_text_option() -> TestResult {
    test_echo_with_options("Hello, world!", "Hello, world!\n", &[])
}

#[test]
fn test_no_newline_option() -> Result<(), Box<dyn std::error::Error>> {
    test_echo_with_options("Hello, world!", "Hello, world!", &["-n"])
}

#[test]
fn test_no_whitespace_option() -> TestResult {
    test_echo_with_options("Hello World", "HelloWorld\n", &["-s"])
}

#[test]
fn test_quote_output_option() -> TestResult {
    test_echo_with_options("Hello World", "'Hello World'\n", &["-q", "single"])
}

#[test]
fn test_quote_style() -> TestResult {
    test_echo_with_options(r#"hello "world""#, "'hello \"world\"'\n", &["-q", "single"])
}

#[test]
fn test_escape_style() -> TestResult {
    test_echo_with_options(r"hello\nworld", "hello\nworld\n", &["-e", "basic"])
}
