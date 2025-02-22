use crate::image::NewSlide;
use crate::path::video_dir_name;
use crate::path::PathStr;
use std::path::Path;

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

fn create_video_clip(dir: &str, slide: &NewSlide) {
    let input_audio = crate::path::audio_path(dir, slide, "mp3");
    let input_image = crate::path::image_path(dir, slide);
    let output_video = crate::path::video_path(dir, slide);
    tracing::info!("Creating video clip {}", output_video.to_string());
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
    } else {
        tracing::info!("Created video clip {}", output_video.to_string());
    }
}

fn create_video_clips(dir: &str, slides: &Vec<NewSlide>) {
    for slide in slides {
        if slide.idx == 0 {
            let output_video = crate::path::video_path(dir, slide);
            let parent = output_video.parent().unwrap();
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
        }
        create_video_clip(dir, slide);
    }
}

pub fn generate_video(dir: &str, slides: &Vec<NewSlide>, output: &str) {
    create_video_clips(dir, slides);
    let output = Path::new(dir).join(output);
    let output = output.to_str().unwrap();
    let concat_list = Path::new(dir).join("concat_list.txt");
    let concat_list = concat_list.to_str().unwrap();
    write_concat_list(dir, concat_list, slides);
    concat_video_clips(concat_list, output);
}
