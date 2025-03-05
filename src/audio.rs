use crate::path::audio_cache_key_path;
use crate::path::audio_path;
use crate::slide::Slide;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use transformrs::text_to_speech::TTSConfig;
use transformrs::Key;
use transformrs::Keys;
use transformrs::Provider;

#[derive(Debug, Serialize, Deserialize)]
struct AudioCacheKey {
    text: String,
    config: TTSConfig,
}

fn write_cache_key(dir: &str, slide: &Slide, config: &TTSConfig) {
    let txt_path = audio_cache_key_path(dir, slide);
    let mut file = File::create(txt_path).unwrap();
    let text = slide.speaker_note.clone();
    let cache_key = AudioCacheKey {
        text,
        config: config.clone(),
    };
    file.write_all(serde_json::to_string(&cache_key).unwrap().as_bytes())
        .unwrap();
}

/// Whether the audio file for the given slide exists and is for the same slide.
fn is_cached(dir: &str, slide: &Slide, config: &TTSConfig, audio_ext: &str) -> bool {
    let txt_path = audio_cache_key_path(dir, slide);
    let audio_path = audio_path(dir, slide, audio_ext);
    if !txt_path.exists() || !audio_path.exists() {
        return false;
    }
    let stored_key = std::fs::read_to_string(txt_path).unwrap();
    let text = slide.speaker_note.clone();
    let cache_key = AudioCacheKey {
        text,
        config: config.clone(),
    };
    let current_info = serde_json::to_string(&cache_key).unwrap();
    stored_key == current_info
}

#[allow(clippy::too_many_arguments)]
async fn generate_audio_file(
    provider: &Provider,
    keys: &Keys,
    dir: &str,
    slide: &Slide,
    cache: bool,
    config: &TTSConfig,
    model: &Option<String>,
    audio_ext: &str,
) {
    fn get_key(keys: &Keys, provider: &Provider) -> Key {
        match keys.for_provider(provider) {
            Some(key) => key,
            None => {
                panic!("no key for provider {:?}", provider);
            }
        }
    }
    let key = match provider {
        Provider::OpenAICompatible(domain) => {
            // Yes the whole key and providers API from transformrs is a mess.
            if domain.contains("kokoros.transformrs.org") {
                Key {
                    provider: Provider::OpenAICompatible(domain.to_string()),
                    key: "test".to_string(),
                }
            } else {
                get_key(keys, provider)
            }
        }
        _ => get_key(keys, provider),
    };
    let msg = &slide.speaker_note;
    let is_cached = cache && is_cached(dir, slide, config, audio_ext);
    if is_cached {
        tracing::info!(
            "Slide {}: Skipping audio generation due to cache",
            slide.idx
        );
        return;
    }
    let model = model.as_deref();
    let resp = transformrs::text_to_speech::tts(provider, &key, config, model, msg)
        .await
        .unwrap()
        .structured()
        .unwrap();
    let bytes = resp.audio.clone();
    let path = audio_path(dir, slide, audio_ext);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
        }
    }
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
    if cache && !is_cached {
        write_cache_key(dir, slide, config);
    }
}

pub async fn generate_audio_files(
    provider: &Provider,
    dir: &str,
    slides: &Vec<Slide>,
    cache: bool,
    config: &TTSConfig,
    model: &Option<String>,
    audio_ext: &str,
) {
    // Not using the keys from file (TODO: transformrs should support loading
    // keys from environment variables).
    let keys = transformrs::load_keys("not_used.env");
    for slide in slides {
        let idx = slide.idx;
        tracing::info!("Slide {idx}: Generating audio file...");
        generate_audio_file(provider, &keys, dir, slide, cache, config, model, audio_ext).await;
    }
}
