use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

const PROGRAM_NAME: &str = "echo_sab";

#[test]
fn test_no_options() -> Result<(), Box<dyn std::error::Error>> {
    let output = "Hello, world!";
    let expected_output = predicate::eq(format!("{}\n", output));
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    let assert = cmd.arg(output).assert();
    assert
        .success()
        .stdout(expected_output)
        .stdout(predicate::str::ends_with("\n"));
    Ok(())
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
