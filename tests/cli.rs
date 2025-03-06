mod common;

use common::bin;
use predicates::prelude::*;
use std::path::Path;
use transformrs::Provider;

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
fn test_cache() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_cache_out");
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");
    let provider = Provider::DeepInfra;
    let key = common::load_key(&provider);

    // Not deleting the dir to avoid cargo watch going into an infinite loop.
    let files = vec![
        "audio/1.mp3",
        "audio/2.mp3",
        "audio/1.audio.cache_key",
        "video/1.mkv",
        "video/2.mkv",
        "video/1.video.cache_key",
        "concat_list.txt",
        "out.mkv",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.env("DEEPINFRA_KEY", &key);
    cmd.arg("--verbose");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("build");
    cmd.arg("tests/test_cache.typ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Slide 1: Generating audio file"))
        .stdout(predicate::str::contains("Skipping").not());

    for file in &files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    println!("Starting second run...");

    let mut cmd = bin();
    cmd.env("DEEPINFRA_KEY", key);
    cmd.arg("build");
    cmd.arg("tests/test_cache.typ");
    cmd.arg("--verbose");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Slide 1: Generating audio file"))
        .stdout(predicate::str::contains(
            "Slide 1: Skipping audio generation due to cache",
        ))
        .stdout(predicate::str::contains(
            "Slide 1: Skipping video generation due to cache",
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
        "audio/1.wav",
        "audio/2.wav",
        "audio/1.audio.cache_key",
        "video/1.mkv",
        "concat_list.txt",
        "out.mkv",
        "release.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("--verbose");
    cmd.arg("--release");
    cmd.arg("build");
    cmd.arg("tests/test_openai_compatible.typ");
    cmd.assert().success();

    for file in &files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    Ok(())
}

#[test]
fn google_provider() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_google_out");
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");
    let key = common::load_key(&Provider::Google);

    // Not deleting the dir to avoid cargo watch going into an infinite loop.
    let files = vec![
        "audio/1.mp3",
        "audio/2.mp3",
        "audio/1.audio.cache_key",
        "video/1.mkv",
        "concat_list.txt",
        "out.mkv",
        "release.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("--verbose");
    cmd.env("GOOGLE_KEY", key);
    cmd.arg("build");
    cmd.arg("tests/test_google.typ");
    if common::is_ci() {
        cmd.arg("--audio-codec=opus");
    } else {
        cmd.arg("--audio-codec=aac_at");
    }
    cmd.arg("--release");
    cmd.assert().success();

    for file in files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    Ok(())
}
