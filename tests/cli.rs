use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_echo_with_newline() -> Result<(), Box<dyn std::error::Error>> {
    let expected_output = "Hello, world!\n";
    let mut cmd = Command::cargo_bin("echo_sab")?;
    let assert = cmd.arg("Hello, world!").assert();
    assert.success().stdout(predicate::eq(expected_output));
    Ok(())
}
