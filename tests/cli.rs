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
fn test_enable_escapes_option() -> TestResult {
    test_echo_with_options("Hello\\tWorld", "Hello\tWorld\n", &["-e"])
}

#[test]
fn test_disable_escapes_option() -> TestResult {
    test_echo_with_options(r#"hello\nworld\t!"#, "hellonworldt!\n", &["-E"])
}

#[test]
fn test_no_whitespace_option() -> TestResult {
    test_echo_with_options("Hello World", "HelloWorld\n", &["-s"])
}

#[test]
fn test_quote_output_option() -> TestResult {
    test_echo_with_options("Hello World", "'Hello World'\n", &["-p"])
}

#[test]
fn test_no_newline_and_enable_escapes_options() -> TestResult {
    test_echo_with_options("Hello\\nWorld", "Hello\nWorld", &["-n", "-e"])
}

#[test]
fn test_disable_escapes_and_quote_output_options() -> TestResult {
    test_echo_with_options("Hello\\nWorld", "'HellonWorld'\n", &["-E", "-p"])
}

#[test]
fn test_no_whitespace_and_quote_output_options() -> TestResult {
    test_echo_with_options("Hello World", "'Hello World'\n", &["-E", "-p"])
}

#[test]
fn test_no_newline_and_no_whitespace_options() -> TestResult {
    test_echo_with_options("Hello World", "HelloWorld", &["-n", "-s"])
}

#[test]
fn test_enable_escapes_and_quote_output_options() -> TestResult {
    test_echo_with_options("Hello\\nWorld", "'Hello\nWorld'\n", &["-e", "-p"])
}
