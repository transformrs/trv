use crate::image::NewSlide;
use crate::path::audio_cache_key_path;
use crate::path::audio_path;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use transformrs::text_to_speech::TTSConfig;
use transformrs::Keys;
use transformrs::Provider;

#[derive(Debug, Serialize, Deserialize)]
struct CacheKey {
    text: String,
    config: TTSConfig,
}

fn write_cache_key(dir: &str, slide: &NewSlide, config: &TTSConfig) {
    let txt_path = audio_cache_key_path(dir, slide);
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
fn is_cached(dir: &str, slide: &NewSlide, config: &TTSConfig, ext: &str) -> bool {
    let txt_path = audio_cache_key_path(dir, slide);
    let mp3_path = audio_path(dir, slide, ext);
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
    let ext = "mp3";
    let key = keys.for_provider(&provider).expect("no key for provider");
    let mut other = HashMap::new();
    other.insert("seed".to_string(), json!(42));
    let config = transformrs::text_to_speech::TTSConfig {
        voice: Some("am_liam".to_string()),
        output_format: Some(ext.to_string()),
        speed: Some(1.25),
        other: Some(other),
        ..Default::default()
    };
    let msg = &slide.note;
    if cache && is_cached(dir, slide, &config, ext) {
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
    let path = audio_path(dir, slide, ext);
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
