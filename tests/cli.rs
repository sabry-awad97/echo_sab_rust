use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

const PROGRAM_NAME: &str = "echo_sab";

#[test]
fn test_echo_with_newline() -> Result<(), Box<dyn std::error::Error>> {
    let expected_output = "Hello, world!\n";
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    let assert = cmd.arg("Hello, world!").assert();
    assert
        .success()
        .stdout(predicate::eq(expected_output))
        .stdout(predicate::str::ends_with("\n"));
    Ok(())
}

#[test]
fn test_echo_without_newline() -> Result<(), Box<dyn std::error::Error>> {
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
