use crate::path::audio_path;
use crate::path::image_path;
use crate::slide::Slide;
use std::path::Path;
use std::path::PathBuf;

// 1920 is the height of a HD YouTube Short.
// It should be a good height for landscape videos too.
// Since the video consists of images, data-wise it should be not a problem to go for a higher resolution.
const HEIGHT: i32 = 1920;

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

fn video_filters(slides: &[Slide]) -> Vec<String> {
    slides
        .iter()
        .map(|slide| {
            let input_index = stream_index(slide, Stream::Video);
            let output_name = video_output_name(slide);
            // For example, `[1:v]scale=-1:1920,format=yuv420p[v1];`.
            format!("[{input_index}:v]scale=-1:{HEIGHT},format=yuv420p[{output_name}];")
        })
        .collect::<Vec<String>>()
}

fn video_inputs(slides: &[Slide]) -> Vec<String> {
    slides
        .iter()
        .map(|slide| {
            let video_output_name = video_output_name(slide);
            let audio_index = stream_index(slide, Stream::Audio);
            // For example, `[v1][0:a]`.
            format!("[{video_output_name}][{audio_index}:a]")
        })
        .collect::<Vec<String>>()
}

enum Stream {
    Audio,
    Video,
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

pub(crate) fn combine_video(
    dir: &str,
    slides: &Vec<Slide>,
    output: &str,
    audio_codec: &str,
    audio_ext: &str,
) {
    tracing::info!("Combining images and audio into one video...");
    let output = Path::new(dir).join(output);
    let output_path = output.to_str().unwrap();

    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y");
    for slide in slides {
        let audio_path = audio_path(dir, slide, audio_ext);
        cmd.arg("-i").arg(&audio_path);
        let image_path = image_path(dir, slide);
        let duration = probe_duration(&audio_path).unwrap();
        cmd.arg("-loop")
            .arg("1")
            .arg("-framerate")
            .arg("1")
            .arg("-t")
            .arg(duration)
            .arg("-i")
            .arg(image_path);
    }
    let filter = format!(
        "{} {} concat=n={}:v=1:a=1 [outv] [outa]",
        video_filters(slides).join(" "),
        video_inputs(slides).join(""),
        slides.len()
    );
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
        // Try to avoid pauses.
        .arg("-shortest")
        // Move some data to the beginning for faster playback start.
        .arg("-movflags")
        .arg("faststart")
        .arg(output_path);
    tracing::debug!("FFmpeg command:\n{:?}", cmd);
    let output = cmd.output().expect("Failed to run ffmpeg command");
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("Failed to concat video clips: {stderr}");
        std::process::exit(1);
    } else {
        tracing::info!("Combined video clips into {output_path}");
    }
}
