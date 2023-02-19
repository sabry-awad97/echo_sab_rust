use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

const PROGRAM_NAME: &'static str = "echo_sab";

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Helper function to test the `echo` command with various options.
fn test_echo_with_options(input: &str, expected_output: &str, options: &[&str]) -> TestResult {
    /* Arrange */
    let predicate_fn = predicate::eq(format!("{}", expected_output));

    /* Act */

    // Create a new `Command` instance for the `echo` program.
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;

    // Add the options to the command.
    for option in options {
        cmd.arg(option);
    }

    // Invoke the `echo` program with the input argument.
    let output = cmd.arg(input).output()?;

    // Convert the output byte string to a UTF-8 string.
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    /* Assert */

    // Assert that the output string matches the expected string.
    assert_eq!(true, predicate_fn.eval(&stdout));

    // Return `Ok(())` to indicate that the test case passed.
    Ok(())
}

#[test]
fn test_text_option() -> TestResult {
    test_echo_with_options("Hello, world!", "Hello, world!\n", &[])
}

#[test]
fn test_no_newline() -> Result<(), Box<dyn std::error::Error>> {
    let expected_output = "Hello, world!";
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    let assert = cmd.arg("-n").arg("Hello, world!").assert();
    assert
        .success()
        .stdout(predicate::eq(expected_output))
        .stdout(predicate::str::ends_with("\n").not());
    Ok(())
}

#[test]
fn test_enable_escapes() {
    let expected_output = "Hello\tWorld\n";
    let mut cmd = Command::cargo_bin(PROGRAM_NAME).unwrap();
    let assert = cmd.arg("-e").arg("Hello\\tWorld").assert();
    assert.success().stdout(expected_output);
}

#[test]
fn test_no_newline_and_enable_escapes() {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME).unwrap();
    let assert = cmd
        .arg("--no-newline")
        .arg("-e")
        .arg("Hello\\tWorld\\n")
        .assert();

    assert
        .success()
        .stdout(predicate::str::contains("\t"))
        .stdout(predicate::str::ends_with("\n"));
}

#[test]
fn test_disable_escapes() {
    let output = r#"hello\nworld\t!"#;
    let expected = predicate::eq("hellonworldt!");

    let mut cmd = Command::cargo_bin(PROGRAM_NAME).unwrap();
    cmd.arg("-n")
        .arg("-E")
        .arg(output)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test_no_whitespace() {
    let output = "Hello World";
    let expected = predicate::eq("HelloWorld\n");

    let mut cmd = Command::cargo_bin(PROGRAM_NAME).unwrap();
    cmd.arg("-s")
        .arg(output)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn test_quote_output() {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME).unwrap();
    cmd.arg("-p")
        .arg("Hello World")
        .assert()
        .success()
        .stdout("'Hello World'\n");
}
