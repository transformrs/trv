use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Slide {
    pub idx: u64,
    pub speaker_note: String,
}

/// Cleanup the speaker note.
///
/// For example, this removes whitespace in front of the text. Kokoro typically
/// ignores the space, but other models like Zyphra Zonos may add random sounds
/// due to the space. To see this, add 4 spaces in front of the Zonos demo at
/// deepinfra.com and see it will add a sound like "aaah". The demo at the
/// Zyphra Playground does not have this issue.
fn trim_speaker_note(text: &str) -> String {
    let lines = text.lines().collect::<Vec<&str>>();
    let trimmed = lines
        .iter()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n");
    let placeholder = "<<<DOUBLE NEWLINE>>>";
    let less_newlines = trimmed
        .replace("\n\n", placeholder)
        .replace("\n", "")
        .replace(placeholder, "\n\n");
    less_newlines.trim().to_string()
}

#[test]
fn test_trim_speaker_note() {
    let text = "\n    foo.\n\nbar.\n\n ";
    let out = trim_speaker_note(text);
    assert_eq!(out, "foo.\n\nbar.");
}

impl Slide {
    fn new(idx: &Value, speaker_note: &Value) -> Self {
        let idx = idx.get("v").and_then(|v| v.as_u64()).unwrap();
        // Typst generates images starting at index 1.
        let idx = idx + 1;
        let speaker_note = speaker_note.get("v").and_then(|v| v.as_str()).unwrap();
        let speaker_note = trim_speaker_note(speaker_note);
        Self { idx, speaker_note }
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
    match serde_json::from_str::<Value>(&text) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Error parsing JSON: {}", e);
            tracing::error!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    }
}

pub fn slides(input: &str) -> Vec<Slide> {
    let json = query_presenter_notes(input);

    let values = json.as_array().expect("Expected JSON array");

    let mut slides = Vec::new();

    for i in 0..values.len() {
        let note = &values[i];
        if let Some(obj) = note.as_object() {
            if let Some(t) = obj.get("t") {
                if t == "NewSlide" {
                    let idx = &values[i + 1];
                    let speaker_note = &values[i + 4];
                    let slide = Slide::new(idx, speaker_note);
                    slides.push(slide);
                }
            }
        }
    }
    slides
}
