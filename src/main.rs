use clap::Parser;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use tracing::subscriber::SetGlobalDefaultError;

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

fn presenter_notes(input: &str) -> Value {
    let output = std::process::Command::new("typst")
        .arg("query")
        .arg(&input)
        .arg("<pdfpc>")
        .arg("--field=value")
        .output()
        .expect("Failed to run typst presenter-notes command");

    let text = String::from_utf8_lossy(&output.stdout);
    let json = serde_json::from_str::<Value>(&text).expect("invalid json");
    json
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

fn presenter_notes_map(input: &str) -> Vec<NewSlide> {
    let json = presenter_notes(input);

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

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    if args.verbose {
        init_subscriber(tracing::Level::DEBUG).unwrap();
    } else {
        init_subscriber(tracing::Level::INFO).unwrap();
    }

    let output = std::process::Command::new("typst")
        .arg("compile")
        .arg(&args.input)
        .output()
        .expect("Failed to run typst compile command");

    if !output.status.success() {
        eprintln!("Error running typst compile:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    let val = presenter_notes(&args.input);
    println!("{:?}", val);

    let slides = presenter_notes_map(&args.input);
    for slide in slides {
        println!("{:?}", slide);
    }
}
