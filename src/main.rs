mod video;

use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tracing::subscriber::SetGlobalDefaultError;
use transformrs::Keys;
use transformrs::Provider;

#[derive(clap::Subcommand)]
enum Commands {}

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
struct Arguments {
    /// Path to the input file.
    #[arg()]
    input: String,

    /// Verbose output.
    ///
    /// The output of the logs is printed to stderr because the output is
    /// printed to stdout.
    #[arg(long)]
    verbose: bool,
}

pub enum Task {
    #[allow(clippy::upper_case_acronyms)]
    TTS,
}

/// Initialize logging with the given level.
fn init_subscriber(level: tracing::Level) -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stderr)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
}

#[derive(Debug)]
struct NewSlide {
    pub idx: u64,
    #[allow(dead_code)]
    pub overlay: u64,
    #[allow(dead_code)]
    pub logical_slide: u64,
    pub note: String,
}

impl NewSlide {
    fn new(idx: &Value, overlay: &Value, logical_slide: &Value, note: &Value) -> Self {
        let idx = idx.get("v").and_then(|v| v.as_u64()).unwrap();
        let overlay = overlay.get("v").and_then(|v| v.as_u64()).unwrap();
        let logical_slide = logical_slide.get("v").and_then(|v| v.as_u64()).unwrap();
        let note = note.get("v").and_then(|v| v.as_str()).unwrap();
        Self {
            idx,
            overlay,
            logical_slide,
            note: note.to_string(),
        }
    }
}

fn query_presenter_notes(input: &str) -> Value {
    let output = std::process::Command::new("typst")
        .arg("query")
        .arg(input)
        .arg("<pdfpc>")
        .arg("--field=value")
        .output()
        .expect("Failed to run typst presenter-notes command");

    let text = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<Value>(&text).expect("invalid json")
}

fn presenter_notes(input: &str) -> Vec<NewSlide> {
    let json = query_presenter_notes(input);

    let values = json.as_array().expect("Expected JSON array");

    let mut slides = Vec::new();

    for i in 0..values.len() {
        let note = &values[i];
        if let Some(obj) = note.as_object() {
            if let Some(t) = obj.get("t") {
                if t == "NewSlide" {
                    let idx = &values[i + 1];
                    let overlay = &values[i + 2];
                    let logical_slide = &values[i + 3];
                    let note = &values[i + 4];
                    let slide = NewSlide::new(idx, overlay, logical_slide, note);
                    slides.push(slide);
                }
            }
        }
    }

    slides
}

fn generate_images(input: &PathBuf, dir: &str) {
    let output = std::process::Command::new("typst")
        .arg("compile")
        .arg("--format=png")
        .arg("--ppi=300")
        .arg(format!("--root={}", dir))
        .arg(input)
        .arg(format!("{}/{{p}}.png", dir))
        .output()
        .expect("Failed to run typst compile");

    if !output.status.success() {
        eprintln!("Error running typst compile:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    if !output.stdout.is_empty() {
        tracing::info!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

async fn generate_audio_file(keys: &Keys, dir: &str, slide: &NewSlide) {
    let provider = Provider::DeepInfra;
    let key = keys.for_provider(&provider).unwrap();
    let config = transformrs::text_to_speech::TTSConfig {
        voice: Some("am_echo".to_string()),
        output_format: Some("mp3".to_string()),
        speed: Some(1.3),
        ..Default::default()
    };
    let msg = &slide.note;
    let model = Some("hexgrad/Kokoro-82M");
    let resp = transformrs::text_to_speech::tts(&key, &config, model, msg)
        .await
        .unwrap()
        .structured()
        .unwrap();
    let bytes = resp.audio.clone();
    let ext = resp.file_format;
    // Typst png files start at one, while slide.idx at zero.
    let idx = slide.idx + 1;
    let path = Path::new(dir).join(format!("{idx}.{ext}"));
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

async fn generate_audio_files(dir: &str, slides: &Vec<NewSlide>) {
    let keys = transformrs::load_keys(".env");
    for slide in slides {
        tracing::info!("Generating audio file for slide {}", slide.idx);
        generate_audio_file(&keys, dir, slide).await;
    }
}

fn copy_input(input: &str, dir: &str) -> PathBuf {
    let path = Path::new(dir).join("input.pdf");
    std::fs::copy(input, &path).unwrap();
    path
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    if args.verbose {
        init_subscriber(tracing::Level::DEBUG).unwrap();
    } else {
        init_subscriber(tracing::Level::INFO).unwrap();
    }

    let dir = "_out";
    let path = Path::new(dir);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    let input = copy_input(&args.input, dir);

    generate_images(&input, dir);
    let slides = presenter_notes(&args.input);
    for slide in &slides {
        println!("{:?}", slide);
    }
    generate_audio_files(dir, &slides).await;
    video::create_video(dir, "out.mp4");
}
