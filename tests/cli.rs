use std::fs;

use anyhow::Result;
use assert_cmd::Command;

const PROGRAM: &str = "myjlox";

#[test]
fn test_environment_block_scope() -> Result<()> {
    let input_file = "tests/input/env_scope.txt";
    let expected_file = "tests/expected/env_scope.txt";

    let args = &[input_file];
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin(PROGRAM)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
