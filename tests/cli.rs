mod common;

use common::bin;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn unexpected_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = bin();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: trv"));

    Ok(())
}

#[test]
fn audio_cache() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_out");
    if out_dir.exists() {
        for entry in std::fs::read_dir(&out_dir)? {
            let entry = entry?;
            let path = entry.path();
            // Only removing the audio files to avoid `cargo watch` going into
            // an infinite loop.
            if path.extension().map_or(false, |ext| ext == "mp3") {
                std::fs::remove_file(path)?;
            }
        }
    }
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");

    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Generating audio file for slide 1",
        ))
        .stdout(predicate::str::contains("Skipping").not());

    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Generating audio file for slide 1",
        ))
        .stdout(predicate::str::contains(
            "Skipping audio generation for slide 1 due to cache",
        ));

    Ok(())
}
