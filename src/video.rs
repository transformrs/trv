use std::path::Path;

fn generate_concat_list(dir: &str) -> String {
    let mut lines = Vec::new();
    let files = std::fs::read_dir(dir).expect("couldn't read dir");
    for file in files {
        let file = file.expect("couldn't read file");
        let path = file.path();
        if let Some(ext) = path.extension() {
            if ext == "mp4" {
                let path = path.file_name().unwrap().to_str().unwrap();
                let line = format!("file '{path}'");
                lines.push(line);
            }
        }
    }
    lines.join("\n")
}

fn write_concat_list(dir: &str, path: &str) {
    let concat_list = generate_concat_list(dir);
    std::fs::write(path, concat_list).expect("couldn't write concat list");
}

fn concat_video_clips(concat_list: &str, output: &str) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-i")
        .arg(concat_list)
        .arg("-c")
        .arg("copy")
        .arg(output)
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to concat video clips: {stderr}");
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        tracing::info!("concat video clips: {stdout}");
    }
}

fn create_video_clip(input_audio: &str, input_image: &str, output: &str) {
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_audio)
        .arg("-i")
        .arg(input_image)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg(output)
        .output()
        .expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to create video clip: {stderr}");
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        tracing::info!("created video clip: {stdout}");
    }
}

fn create_video_clips(dir: &str) {
    let files = std::fs::read_dir(dir).expect("couldn't read dir");
    for file in files {
        let file = file.expect("couldn't read file");
        let path = file.path();
        if let Some(ext) = path.extension() {
            if ext == "mp3" {
                let input_audio = path.to_str().expect("couldn't convert path to string");
                let input_image = &input_audio.replace(".mp3", ".png");
                let output = &input_audio.replace(".mp3", ".mp4");
                create_video_clip(input_audio, input_image, output);
            }
        }
    }
}

pub fn create_video(dir: &str, output: &str) {
    create_video_clips(dir);
    let output = Path::new(dir).join(output);
    let output = output.to_str().unwrap();
    let concat_list = Path::new(dir).join("concat_list.txt");
    let concat_list = concat_list.to_str().unwrap();
    write_concat_list(dir, concat_list);
    concat_video_clips(concat_list, output);
}
