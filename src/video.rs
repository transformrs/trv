use crate::path::image_path;
use crate::path::video_cache_key_path;
use crate::path::video_dir_name;
use std::path::PathBuf;
use crate::path::video_path;
use crate::path::audio_path;
use crate::path::PathStr;
use crate::slide::Slide;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use transformrs::text_to_speech::TTSConfig;

#[derive(Debug, Serialize, Deserialize)]
struct VideoCacheKey {
    slide: Slide,
    config: TTSConfig,
    image_hash: Vec<u8>,
}

fn hash_file(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_ok() {
        hasher.update(&buffer);
    }
    hasher.finalize().to_vec()
}

fn write_cache_key(dir: &str, slide: &Slide, config: &TTSConfig) {
    let image_path = image_path(dir, slide);
    let image_hash = hash_file(&image_path);

    let cache_key = VideoCacheKey {
        slide: slide.clone(),
        config: config.clone(),
        image_hash,
    };
    let output_path = video_cache_key_path(dir, slide);
    let mut file = File::create(output_path).unwrap();
    file.write_all(serde_json::to_string(&cache_key).unwrap().as_bytes())
        .unwrap();
}

fn is_cached(dir: &str, slide: &Slide, config: &TTSConfig) -> bool {
    let key_path = video_cache_key_path(dir, slide);
    let video_path = video_path(dir, slide);
    if !key_path.exists() || !video_path.exists() {
        return false;
    }
    let stored_key = std::fs::read_to_string(key_path).unwrap();
    let image_path = image_path(dir, slide);
    let image_hash = hash_file(&image_path);
    let cache_key = VideoCacheKey {
        slide: slide.clone(),
        config: config.clone(),
        image_hash,
    };
    let current_info = serde_json::to_string(&cache_key).unwrap();
    stored_key == current_info
}

fn generate_concat_list(dir: &str, slides: &Vec<Slide>) -> String {
    let mut lines = Vec::new();
    for slide in slides {
        let path = crate::path::video_path(dir, slide);
        let filename = path.file_name().unwrap();
        let path = Path::new(video_dir_name()).join(filename);
        let path = path.to_string();
        let line = format!("file '{path}'");
        lines.push(line);
    }
    // Do not sort the lines here or concat will place 10 and 11 before 2!
    // lines.sort();
    lines.join("\n")
}

fn set_default_ffmpeg_video_args(cmd: &mut std::process::Command) {
    cmd.arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("23")
        .arg("-preset")
        .arg("fast")
        .arg("-vf")
        .arg(format!("scale=-1:{HEIGHT},format=yuv420p"))
        .arg("-pix_fmt")
        .arg("yuv420p");
}

fn write_concat_list(dir: &str, path: &str, slides: &Vec<Slide>) {
    let concat_list = generate_concat_list(dir, slides);
    std::fs::write(path, concat_list).expect("couldn't write concat list");
}

// 1920 is the height of a HD YouTube Short.
// It should be a good height for landscape videos too.
// Since the video consists of images, data-wise it should be not a problem to go for a higher resolution.
const HEIGHT: i32 = 1920;

fn create_video_clip(dir: &str, slide: &Slide, cache: bool, config: &TTSConfig, ext: &str) {
    tracing::info!("Slide {}: Generating video file...", slide.idx);
    let input_audio = crate::path::audio_path(dir, slide, ext);
    let input_image = crate::path::image_path(dir, slide);
    let is_cached = cache && is_cached(dir, slide, config);
    if is_cached {
        tracing::info!(
            "Slide {}: Skipping video generation due to cache",
            slide.idx
        );
        return;
    }
    let output_video = crate::path::video_path(dir, slide);
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y")
        .arg("-loop")
        .arg("1")
        .arg("-i")
        .arg(input_image)
        .arg("-i")
        .arg(input_audio);
    set_default_ffmpeg_video_args(&mut cmd);
    cmd.arg("-c:a")
        .arg("opus")
        .arg("-strict")
        .arg("experimental")
        .arg("-shortest")
        .arg("-tune")
        .arg("stillimage")
        .arg(output_video.clone());
    let output = cmd.output().expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to create video clip: {stderr}");
        std::process::exit(1);
    }
    if cache && !is_cached {
        write_cache_key(dir, slide, config);
    }
}

pub(crate) fn create_video_clips(
    dir: &str,
    slides: &Vec<Slide>,
    cache: bool,
    config: &TTSConfig,
    audio_ext: &str,
) {
    {
        let slide = slides.first().unwrap();
        let output_video = crate::path::video_path(dir, slide);
        let parent = output_video.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
        }
    }
    for slide in slides {
        create_video_clip(dir, slide, cache, config, audio_ext);
    }
}

fn probe_duration(path: &PathBuf) -> Option<String> {
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

fn video_output_name(slide: &Slide) -> String {
    // For example, the first video will be called v1, the second v2, etc.
    format!("v{}", slide.idx)
}

enum Stream {
    Audio,
    Video
}

fn stream_index(slide: &Slide, stream: Stream) -> usize {
    // For example, the first stream will be at 0, the second at 2, etc.
    let index = 2 * (slide.idx - 1) as usize;
    match stream {
        // For example, the first audio is at index 0, the second at index 2, etc.
        Stream::Audio => index,
        // For example, the first image is at index 1:v, the second at index 3:v, etc.
        Stream::Video => index + 1,
    }
}

pub(crate) fn combine_video(dir: &str, slides: &Vec<Slide>, output: &str, audio_codec: &str, audio_ext: &str) {
    tracing::info!("Combining video clips into one video...");
    let output = Path::new(dir).join(output);
    let output_path = output.to_str().unwrap();
    let concat_list = Path::new(dir).join("concat_list.txt");
    let concat_list = concat_list.to_str().unwrap();
    write_concat_list(dir, concat_list, slides);

    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y");
    for slide in slides {
        let audio_path = audio_path(dir, slide, audio_ext);
        cmd.arg("-i").arg(&audio_path);
        let image_path = image_path(dir, slide);
        let duration = probe_duration(&audio_path).unwrap();
        cmd.arg("-loop").arg("1").arg("-framerate").arg("1").arg("-t").arg(duration).arg("-i").arg(image_path);
    }
    let video_filters = slides.iter().map(|slide| {
        let input_index = stream_index(slide, Stream::Video);
        let output_name = video_output_name(slide);
        format!("[{input_index}:v]scale=-1:{HEIGHT},format=yuv420p[{output_name}];")
    }).collect::<Vec<String>>();
    let inputs = slides.iter().map(|slide| {
        let video_output_name = video_output_name(slide);
        let audio_index = stream_index(slide, Stream::Audio);
        format!("[{video_output_name}][{audio_index}:a]")
    }).collect::<Vec<String>>();
    let filter = format!("{} {} concat=n=2:v=1:a=1 [outv] [outa]", video_filters.join(" "), inputs.join(""));
    tracing::debug!("Video filter: {}", filter);
    cmd.arg("-filter_complex")
        .arg(filter)
    .arg("-map")
    .arg("[outv]")
    // Re-encode to 30 fps since most video players don't like 1 fps.
    .arg("-r")
    .arg("30")
    .arg("-map")
    .arg("[outa]")
    .arg("-tune")
    .arg("stillimage")
    // Experimental is required for opus.
    .arg("-strict")
    .arg("-2")
    // Default audio codec is aac which has poor quality.
    .arg("-c:a")
    .arg(audio_codec)
    // Move some data to the beginning for faster playback start.
    .arg("-movflags")
    .arg("faststart")
    .arg(output_path);
    tracing::debug!("FFmpeg command:\n{:?}", cmd);
    let output = cmd
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to concat video clips: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Combined video clips into {output_path}");
    }
}
