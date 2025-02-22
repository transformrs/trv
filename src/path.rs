use crate::image::NewSlide;
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

pub fn idx(slide: &NewSlide) -> u64 {
    // Typst png files start at one, while slide.idx at zero.
    slide.idx + 1
}

pub fn audio_path(dir: &str, slide: &NewSlide, ext: &str) -> PathBuf {
    let idx = idx(slide);
    Path::new(dir).join(format!("{idx}.{ext}"))
}

pub fn image_path(dir: &str, slide: &NewSlide) -> PathBuf {
    let idx = idx(slide);
    Path::new(dir).join(format!("{idx}.png"))
}

pub fn audio_cache_key_path(dir: &str, slide: &NewSlide) -> PathBuf {
    let idx = idx(slide);
    Path::new(dir).join(format!("{idx}.audio.cache"))
}

pub fn video_path(dir: &str, slide: &NewSlide) -> PathBuf {
    let idx = idx(slide);
    Path::new(dir).join(format!("{idx}.mp4"))
}
