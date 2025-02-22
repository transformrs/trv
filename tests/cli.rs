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
        "1.mp3",
        "2.mp3",
        "1.audio.cache",
        "1.mp4",
        "concat_list.txt",
        "out.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }
    let audio_2_path = Path::new(out_dir).join("2.mp3");
    if audio_2_path.exists() {
        std::fs::remove_file(&audio_2_path)?;
    }
    let cache_key_path = Path::new(out_dir).join("1.audio.cache");
    if cache_key_path.exists() {
        std::fs::remove_file(&cache_key_path)?;
    }
    let video_path = Path::new(out_dir).join("1.mp4");
    if video_path.exists() {
        std::fs::remove_file(&video_path)?;
    }

    let mut cmd = bin();
    cmd.arg("--input=tests/test.typ");
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
