use crate::idx;
use crate::image::NewSlide;
use std::path::Path;

fn generate_concat_list(dir: &str, video_clips: &Vec<String>) -> String {
    let mut lines = Vec::new();
    for video_clip in video_clips {
        let path = Path::new(dir).join(video_clip);
        let filename = path.file_name().unwrap().to_str().unwrap();
        let line = format!("file '{filename}'");
        lines.push(line);
    }
    lines.sort();
    lines.join("\n")
}

fn write_concat_list(dir: &str, path: &str, video_clips: &Vec<String>) {
    let concat_list = generate_concat_list(dir, video_clips);
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

fn create_video_clip(input_audio: &str, input_image: &str, output_path: &str) {
    tracing::info!("Creating video clip {output_path}");
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
        .arg(output_path)
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to create video clip: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Created video clip {output_path}");
    }
}

fn create_video_clips(dir: &str, slides: &Vec<NewSlide>) -> Vec<String> {
    let mut video_clips = Vec::new();
    for slide in slides {
        let idx = idx(slide);
        let input_audio = Path::new(dir).join(format!("{idx}.mp3"));
        let input_image = Path::new(dir).join(format!("{idx}.png"));
        let output = Path::new(dir).join(format!("{idx}.mp4"));
        create_video_clip(
            input_audio.to_str().unwrap(),
            input_image.to_str().unwrap(),
            output.to_str().unwrap(),
        );
        video_clips.push(output.to_str().unwrap().to_string());
    }
    video_clips
}

pub fn generate_video(dir: &str, slides: &Vec<NewSlide>, output: &str) {
    let video_clips = create_video_clips(dir, slides);
    let output = Path::new(dir).join(output);
    let output = output.to_str().unwrap();
    let concat_list = Path::new(dir).join("concat_list.txt");
    let concat_list = concat_list.to_str().unwrap();
    write_concat_list(dir, concat_list, &video_clips);
    concat_video_clips(concat_list, output);
}
