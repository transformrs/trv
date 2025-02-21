use crate::image::NewSlide;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use transformrs::Keys;
use transformrs::Provider;

async fn generate_audio_file(keys: &Keys, dir: &str, slide: &NewSlide) {
    let provider = Provider::DeepInfra;
    let key = keys.for_provider(&provider).unwrap();
    let mut other = HashMap::new();
    other.insert("seed".to_string(), json!(42));
    let config = transformrs::text_to_speech::TTSConfig {
        voice: Some("am_liam".to_string()),
        output_format: Some("mp3".to_string()),
        speed: Some(1.3),
        other: Some(other),
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

pub async fn generate_audio_files(dir: &str, slides: &Vec<NewSlide>) {
    let keys = transformrs::load_keys(".env");
    for slide in slides {
        tracing::info!("Generating audio file for slide {}", slide.idx);
        generate_audio_file(&keys, dir, slide).await;
    }
}
