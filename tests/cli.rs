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
        "out.mp4",
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
    cmd.arg("--verbose");
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("build");
    cmd.arg("tests/test_cache.typ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Slide 1: Generating audio file"))
        .stdout(predicate::str::contains(
            "Slide 1: Skipping audio generation due to cache",
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
        "out.mp4",
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
fn elevenlabs_provider() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_elevenlabs_out");
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");
    let key = common::load_key(&Provider::ElevenLabs);

    // Not deleting the dir to avoid cargo watch going into an infinite loop.
    let files = vec![
        "audio/1.mp3",
        "audio/2.mp3",
        "audio/1.audio.cache_key",
        "out.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.env("ELEVENLABS_KEY", key);
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("--verbose");
    cmd.arg("build");
    cmd.arg("tests/test_elevenlabs.typ");
    if common::is_ci() {
        cmd.arg("--audio-codec=opus");
    } else {
        cmd.arg("--audio-codec=aac_at");
    }
    cmd.assert().success();

    for file in files {
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
        "out.mp4",
    ];
    for file in &files {
        let path = Path::new(out_dir).join(file);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    let mut cmd = bin();
    cmd.env("GOOGLE_KEY", key);
    cmd.arg(format!("--out-dir={}", out_dir));
    cmd.arg("--verbose");
    cmd.arg("build");
    cmd.arg("tests/test_google.typ");
    if common::is_ci() {
        cmd.arg("--audio-codec=opus");
    } else {
        cmd.arg("--audio-codec=aac_at");
    }
    cmd.assert().success();

    for file in files {
        let path = Path::new(out_dir).join(file);
        assert!(path.exists(), "file {} does not exist", file);
    }

    Ok(())
}

fn convert_to_mp3(input: &str, output: &str) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-q:a")
        .arg("0")
        .arg("-map")
        .arg("a")
        .arg(output)
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to convert to mp3: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Converted to mp3");
    }
}

fn probe_duration(path: &str) -> Option<String> {
    let output = std::process::Command::new("ffprobe")
        .arg("-i")
        .arg(path)
        .output()
        .expect("Failed to run ffprobe command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Failed to probe duration: {stderr}");
        return None;
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let duration = stderr
        .split("Duration: ")
        .nth(1)
        .unwrap()
        .split(",")
        .next()
        .unwrap();
    Some(duration.to_string())
}

fn duration_as_seconds(duration: &str) -> f32 {
    let parts = duration.split(":").collect::<Vec<&str>>();
    let hours = parts[0].parse::<f32>().unwrap();
    let minutes = parts[1].parse::<f32>().unwrap();
    let seconds = parts[2].parse::<f32>().unwrap();
    hours * 3600.0 + minutes * 60.0 + seconds
}

#[test]
fn test_duration_matches() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new("tests").join("_duration_matches_out");
    let out_dir = out_dir.to_str().unwrap();
    println!("out_dir: {out_dir}");

    let key = common::load_key(&Provider::Google);
    let mut cmd = bin();
    cmd.env("GOOGLE_KEY", key);
    cmd.arg(format!("--out-dir={}", out_dir));
    // cmd.arg("--verbose");
    cmd.arg("--cache=false");
    cmd.arg("build");
    cmd.arg("tests/test_duration_matches.typ");
    cmd.assert().success();

    let video_path = Path::new(out_dir).join("out.mp4");
    let video_path = video_path.to_str().unwrap();
    let audio_path = Path::new(out_dir).join("out.mp3");
    let audio_path = audio_path.to_str().unwrap();
    convert_to_mp3(&video_path, &audio_path);

    let video_duration = probe_duration(&video_path).unwrap();
    println!("video_duration: {video_duration}");
    let video_duration = duration_as_seconds(&video_duration);
    println!("video_duration: {video_duration} seconds");
    let audio_duration = probe_duration(&audio_path).unwrap();
    println!("audio_duration: {audio_duration}");
    let audio_duration = duration_as_seconds(&audio_duration);
    println!("audio_duration: {audio_duration} seconds");
    assert!(video_duration - audio_duration < 0.01);

    Ok(())
}

#[test]
fn test_notes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = bin();
    cmd.arg("notes");
    cmd.arg("tests/test_notes.typ");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "What if you could show code in a video?",
        ))
        .stdout(predicate::str::contains("For example, take this code."));

    Ok(())
}
