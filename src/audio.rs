use crate::image::NewSlide;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use transformrs::text_to_speech::TTSConfig;
use transformrs::Keys;
use transformrs::Provider;

fn idx(slide: &NewSlide) -> u64 {
    // Typst png files start at one, while slide.idx at zero.
    slide.idx + 1
}

#[derive(Debug, Serialize, Deserialize)]
struct CacheKey {
    text: String,
    config: TTSConfig,
}

fn write_cache_key(dir: &str, slide: &NewSlide, config: &TTSConfig) {
    let idx = idx(slide);
    let txt_path = Path::new(dir).join(format!("{idx}.txt"));
    let mut file = File::create(txt_path).unwrap();
    let text = slide.note.clone();
    let cache_key = CacheKey {
        text,
        config: config.clone(),
    };
    file.write_all(serde_json::to_string(&cache_key).unwrap().as_bytes())
        .unwrap();
}

/// Whether the audio file for the given slide exists and is for the same slide.
fn is_cached(dir: &str, slide: &NewSlide, config: &TTSConfig) -> bool {
    let idx = idx(slide);
    let txt_path = Path::new(dir).join(format!("{idx}.txt"));
    let mp3_path = Path::new(dir).join(format!("{idx}.mp3"));
    if !txt_path.exists() || !mp3_path.exists() {
        return false;
    }
    let contents = std::fs::read_to_string(txt_path).unwrap();
    let text = slide.note.clone();
    let cache_key = CacheKey {
        text,
        config: config.clone(),
    };
    let serialized = serde_json::to_string(&cache_key).unwrap();
    contents == serialized
}

async fn generate_audio_file(keys: &Keys, dir: &str, slide: &NewSlide, cache: bool) {
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
    if cache && is_cached(dir, slide, &config) {
        tracing::info!(
            "Skipping audio generation for slide {} due to cache",
            slide.idx
        );
        return;
    }
    let model = Some("hexgrad/Kokoro-82M");
    let resp = transformrs::text_to_speech::tts(&key, &config, model, msg)
        .await
        .unwrap()
        .structured()
        .unwrap();
    let bytes = resp.audio.clone();
    let ext = resp.file_format;
    let idx = idx(slide);
    let path = Path::new(dir).join(format!("{idx}.{ext}"));
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
    if cache {
        write_cache_key(dir, slide, &config);
    }
}

pub async fn generate_audio_files(dir: &str, slides: &Vec<NewSlide>, cache: bool) {
    let keys = transformrs::load_keys(".env");
    for slide in slides {
        tracing::info!("Generating audio file for slide {}", slide.idx);
        generate_audio_file(&keys, dir, slide, cache).await;
    }
}
