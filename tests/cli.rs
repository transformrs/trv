mod common;

use common::bin;
use predicates::prelude::*;

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = bin();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: tv"));

    Ok(())
}

#[test]
fn full() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--out-dir=tests/_out");
    cmd.assert().success().stdout(predicate::str::contains(
        "Generating audio file for slide 0",
    ));

    Ok(())
}
