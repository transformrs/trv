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
fn audio_cache() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_out");
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
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--audio-format=mp3");
    cmd.arg("--verbose");
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

    println!("Starting second run...");

    let mut cmd = bin();
    cmd.env("DEEPINFRA_KEY", key);
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--audio-format=mp3");
    cmd.arg("--verbose");
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
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--provider=openai-compatible(kokoros.transformrs.org)");
    cmd.arg("--model=tts-1");
    cmd.arg("--voice=bm_lewis");
    cmd.arg("--audio-format=wav");
    cmd.arg("--release");
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
    cmd.arg("--input=tests/test.typ");
    cmd.arg("--provider=google");
    cmd.arg("--voice=en-US-Studio-Q");
    cmd.arg("--language-code=en-US");
    cmd.arg("--release");
    cmd.assert().success();

    for file in &files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    Ok(())
}
