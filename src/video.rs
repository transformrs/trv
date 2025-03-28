use crate::audio_format;
use crate::path::audio_path;
use crate::path::image_path;
use crate::slide::Slide;
use crate::Config;
use crate::Provider;
use chrono::NaiveTime;
use chrono::SubsecRound;
use chrono::Timelike;
use std::path::Path;
use std::path::PathBuf;

// 1920 is the height of a HD YouTube Short.
// It should be a good height for landscape videos too.
// Since the video consists of images, data-wise it should be not a problem to go for a higher resolution.
const HEIGHT: i32 = 1920;

/// Parse the duration from the output of `ffprobe`.
///
/// See https://ffmpeg.org/ffmpeg-utils.html#time-duration-syntax.
fn parse_ffmpeg_duration(duration: &str) -> NaiveTime {
    let parts: Vec<&str> = duration.split(":").collect();
    let hour = parts[0].parse::<u32>().unwrap();
    let min = parts[1].parse::<u32>().unwrap();
    let last_parts = parts[2].split(".").collect::<Vec<&str>>();
    let sec = last_parts[0].parse::<u32>().unwrap();
    let fraction = format!("0.{}", last_parts[1]);
    let second_fraction = fraction.parse::<f64>().unwrap();
    let milli = (second_fraction * 1000.0) as u32;
    chrono::NaiveTime::from_hms_milli_opt(hour, min, sec, milli).unwrap()
}

#[test]
fn test_parse_ffprobe_duration() {
    assert_eq!(
        parse_ffmpeg_duration("00:00:00.50"),
        NaiveTime::from_hms_milli_opt(0, 0, 0, 500).unwrap()
    );
    assert_eq!(
        parse_ffmpeg_duration("00:00:01.45"),
        NaiveTime::from_hms_milli_opt(0, 0, 1, 450).unwrap()
    );
    assert_eq!(
        parse_ffmpeg_duration("00:01:00.45"),
        NaiveTime::from_hms_milli_opt(0, 1, 0, 450).unwrap()
    );
    assert_eq!(
        parse_ffmpeg_duration("01:00:00.45"),
        NaiveTime::from_hms_milli_opt(1, 0, 0, 450).unwrap()
    );
    assert_eq!(
        parse_ffmpeg_duration("01:00:00.99"),
        NaiveTime::from_hms_milli_opt(1, 0, 0, 990).unwrap()
    );
}

fn probe_duration(path: &PathBuf) -> Option<NaiveTime> {
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
    Some(parse_ffmpeg_duration(duration))
}

fn print_ffmpeg_duration(duration: &NaiveTime) -> String {
    let hour = duration.hour();
    let min = duration.minute();
    let sec = duration.second();
    let subsecs = duration.round_subsecs(9);
    let milli = subsecs.nanosecond() as f64 / 10_000_000.0;
    format!("{hour:02}:{min:02}:{sec:02}.{milli:02}")
}

#[test]
fn test_print_ffmpeg_duration() {
    assert_eq!(
        print_ffmpeg_duration(&NaiveTime::from_hms_milli_opt(0, 0, 0, 500).unwrap()),
        "00:00:00.50"
    );
    assert_eq!(
        print_ffmpeg_duration(&NaiveTime::from_hms_milli_opt(0, 0, 10, 10).unwrap()),
        "00:00:10.01"
    );
    assert_eq!(
        print_ffmpeg_duration(&NaiveTime::from_hms_milli_opt(0, 0, 10, 990).unwrap()),
        "00:00:10.99"
    );
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
    let index = 2 * (slide.idx - 1);
    match stream {
        // For example, the first audio is at index 0, the second at index 2, etc.
        Stream::Audio => index,
        // For example, the first image is at index 1:v, the second at index 3:v, etc.
        Stream::Video => index + 1,
    }
}

/// Pause duration for transitions.
///
/// Sentences normally have a pause between them. Without this pause,
/// sentences around slide transitions will be too close to each other.
/// According to Goldman-Eisler (1968), articulatory pauses are typically
/// below 250 ms while hesitation pauses are typically above that.
fn transition_pause(config: &Config, provider: &Provider) -> chrono::Duration {
    // Google does not automatically have a pause between audio clips.
    if provider == &Provider::Google {
        return chrono::Duration::milliseconds(200);
    }
    if provider == &Provider::ElevenLabs {
        // This should be handled by the previous and next text.
        return chrono::Duration::milliseconds(0);
    }
    if let Some(model) = &config.model {
        // Nor does the Zyphra Zonos model.
        if model.to_lowercase().contains("zonos") {
            return chrono::Duration::milliseconds(200);
        }
        if model.to_lowercase().contains("kokoro") {
            // Most of the time, the pause is fine in Kokoro, but not always.
            return chrono::Duration::milliseconds(50);
        }
    }
    // A very small pause (1/4th of a sentence break) just to be sure.
    chrono::Duration::milliseconds(50)
}

pub(crate) fn combine_video(
    dir: &str,
    slides: &[Slide],
    config: &Config,
    provider: &Provider,
    output: &str,
    audio_codec: &str,
) {
    let audio_ext = audio_format(config);
    tracing::info!("Combining images and audio into one video...");
    let output = Path::new(dir).join(output);
    let output_path = output.to_str().unwrap();

    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-y");
    let n = slides.len();
    for (i, slide) in slides.iter().enumerate() {
        let audio_path = audio_path(dir, slide, &audio_ext);
        cmd.arg("-i").arg(&audio_path);
        let image_path = image_path(dir, slide);
        let pause = if i < n - 1 {
            transition_pause(config, provider)
        } else {
            // Sometimes the audio is trimmed at the end. Adding a small pause
            // to avoid this.
            chrono::Duration::milliseconds(500)
        };
        let duration = probe_duration(&audio_path).unwrap() + pause;
        cmd.arg("-loop")
            .arg("1")
            .arg("-framerate")
            .arg("1")
            .arg("-t")
            .arg(print_ffmpeg_duration(&duration))
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
