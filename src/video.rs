use crate::image::NewSlide;
use crate::path::idx;
use crate::path::video_cache_key_path;
use crate::path::video_dir_name;
use crate::path::video_path;
use crate::path::PathStr;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use transformrs::text_to_speech::TTSConfig;

#[derive(Debug, Serialize, Deserialize)]
struct VideoCacheKey {
    slide: NewSlide,
    config: TTSConfig,
}

fn write_cache_key(dir: &str, slide: &NewSlide, config: &TTSConfig) {
    let path = video_cache_key_path(dir, slide);
    let mut file = File::create(path).unwrap();
    let cache_key = VideoCacheKey {
        slide: slide.clone(),
        config: config.clone(),
    };
    file.write_all(serde_json::to_string(&cache_key).unwrap().as_bytes())
        .unwrap();
}

fn is_cached(dir: &str, slide: &NewSlide, config: &TTSConfig) -> bool {
    let key_path = video_cache_key_path(dir, slide);
    let video_path = video_path(dir, slide);
    if !key_path.exists() || !video_path.exists() {
        return false;
    }
    let stored_key = std::fs::read_to_string(key_path).unwrap();
    let cache_key = VideoCacheKey {
        slide: slide.clone(),
        config: config.clone(),
    };
    let current_info = serde_json::to_string(&cache_key).unwrap();
    stored_key == current_info
}

fn generate_concat_list(dir: &str, slides: &Vec<NewSlide>) -> String {
    let mut lines = Vec::new();
    for slide in slides {
        let path = crate::path::video_path(dir, slide);
        let filename = path.file_name().unwrap();
        let path = Path::new(video_dir_name()).join(filename);
        let path = path.to_string();
        let line = format!("file '{path}'");
        lines.push(line);
    }
    lines.sort();
    lines.join("\n")
}

fn write_concat_list(dir: &str, path: &str, slides: &Vec<NewSlide>) {
    let concat_list = generate_concat_list(dir, slides);
    std::fs::write(path, concat_list).expect("couldn't write concat list");
}

fn create_video_clip(dir: &str, slide: &NewSlide, cache: bool, config: &TTSConfig, ext: &str) {
    let input_audio = crate::path::audio_path(dir, slide, ext);
    let input_image = crate::path::image_path(dir, slide);
    let is_cached = cache && is_cached(dir, slide, config);
    if is_cached {
        tracing::info!(
            "Slide {}: Skipping video generation due to cache",
            idx(slide)
        );
        return;
    }
    let output_video = crate::path::video_path(dir, slide);
    let idx = idx(slide);
    tracing::info!(
        "Slide {idx}: Creating video clip {}",
        output_video.to_string()
    );
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-loop")
        .arg("1")
        .arg("-i")
        .arg(input_image)
        .arg("-i")
        .arg(input_audio)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("copy")
        .arg("-shortest")
        .arg("-tune")
        .arg("stillimage")
        .arg(output_video.clone())
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to create video clip: {stderr}");
        std::process::exit(1);
    }
    tracing::info!(
        "Slide {idx}: Created video clip {}",
        output_video.to_string()
    );
    if cache && !is_cached {
        write_cache_key(dir, slide, config);
    }
}

fn create_video_clips(
    dir: &str,
    slides: &Vec<NewSlide>,
    cache: bool,
    config: &TTSConfig,
    audio_ext: &str,
) {
    for slide in slides {
        if slide.idx == 0 {
            let output_video = crate::path::video_path(dir, slide);
            let parent = output_video.parent().unwrap();
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
        }
        create_video_clip(dir, slide, cache, config, audio_ext);
    }
}

fn concat_video_clips(concat_list: &str, output_path: &str) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg(concat_list)
        .arg("-c")
        .arg("copy")
        .arg(output_path)
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to concat video clips: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Concatenated video clips into {output_path}");
    }
}

pub fn generate_video(
    dir: &str,
    slides: &Vec<NewSlide>,
    cache: bool,
    config: &TTSConfig,
    output: &str,
    audio_ext: &str,
) {
    create_video_clips(dir, slides, cache, config, audio_ext);
    let output = Path::new(dir).join(output);
    let output = output.to_str().unwrap();
    let concat_list = Path::new(dir).join("concat_list.txt");
    let concat_list = concat_list.to_str().unwrap();
    write_concat_list(dir, concat_list, slides);
    concat_video_clips(concat_list, output);
}

pub fn generate_release_video(dir: &str, input: &str, output: &str, audio_codec: &str) {
    let input_path = Path::new(dir).join(input);
    let output_path = Path::new(dir).join(output);
    let output_path = output_path.to_str().unwrap();
    let mut cmd = std::process::Command::new("ffmpeg");
    // 1920 is the height of a HD YouTube Short.
    // It should be a good height for landscape videos too.
    // Since the video consists of images, data-wise it should be not a problem to go for a higher resolution.
    let height = 1920;
    let output = cmd
        .arg("-y")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("23")
        .arg("-preset")
        .arg("fast")
        .arg("-vf")
        .arg(format!("scale=-1:{height},format=yuv420p"))
        .arg("-c:a")
        .arg(audio_codec)
        .arg("-strict")
        .arg("experimental")
        .arg(output_path)
        .output()
        .expect("Failed to run ffmpeg command");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to create release video: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Created release video {}", output_path);
    }
}
