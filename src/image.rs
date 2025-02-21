use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewSlide {
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

pub fn presenter_notes(input: &str) -> Vec<NewSlide> {
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

pub fn generate_images(input: &PathBuf, dir: &str) {
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
