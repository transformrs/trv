use crate::slide::Slide;
use std::path::Path;
use std::path::PathBuf;

pub trait PathStr {
    fn to_string(&self) -> String;
}

impl PathStr for PathBuf {
    fn to_string(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}

pub fn audio_path(dir: &str, slide: &Slide, audio_ext: &str) -> PathBuf {
    let idx = slide.idx;
    let filename = format!("{idx}.{audio_ext}");
    Path::new(dir).join("audio").join(filename)
}

pub fn image_path(dir: &str, slide: &Slide) -> PathBuf {
    let idx = slide.idx;
    let filename = format!("{idx}.png");
    Path::new(dir).join("image").join(filename)
}

pub fn audio_cache_key_path(dir: &str, slide: &Slide) -> PathBuf {
    let idx = slide.idx;
    let filename = format!("{idx}.audio.cache_key");
    Path::new(dir).join("audio").join(filename)
}

pub fn video_cache_key_path(dir: &str, slide: &Slide) -> PathBuf {
    let idx = slide.idx;
    let filename = format!("{idx}.video.cache_key");
    Path::new(dir).join("video").join(filename)
}

pub fn video_dir_name() -> &'static str {
    "video"
}

pub fn video_path(dir: &str, slide: &Slide) -> PathBuf {
    let idx = slide.idx;
    // Using mkv by default because it supports more audio formats.
    let filename = format!("{idx}.mkv");
    Path::new(dir).join(video_dir_name()).join(filename)
}
