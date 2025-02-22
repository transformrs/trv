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
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");

    // Not deleting the dir to avoid cargo watch going into an infinite loop.
    let files = vec![
        "audio/1.opus",
        "audio/2.opus",
        "audio/1.audio.cache_key",
        "video/1.mp4",
        "concat_list.txt",
        "out.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--audio-format=opus");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Generating audio file for slide 1",
        ))
        .stdout(predicate::str::contains("Skipping").not());

    for file in &files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

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

#[test]
fn openai_compatible_provider() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_compatible_out");
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");

    // Not deleting the dir to avoid cargo watch going into an infinite loop.
    let files = vec![
        "audio/1.mp3",
        "audio/2.mp3",
        "audio/1.audio.cache_key",
        "video/1.mp4",
        "concat_list.txt",
        "out.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.env("OPENAI_COMPATIBLE_KEY", "test");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("--verbose");
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--provider=openai-compatible(kokoros.transformrs.org)");
    cmd.arg("--model=tts-1");
    cmd.arg("--audio-format=mp3");
    cmd.assert().success();

    for file in &files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    Ok(())
}
