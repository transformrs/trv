mod common;

use common::bin;
use predicates::prelude::*;
use tempfile::tempdir;

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
fn cache() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = tempdir()?;

    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
    cmd.arg(format!("--out-dir={}", out_dir.path().to_str().unwrap()));
    cmd.assert().success().stdout(predicate::str::contains(
        "Generating audio file for slide 0",
    ));

    Ok(())
}
