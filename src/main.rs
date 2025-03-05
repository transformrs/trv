mod audio;
mod image;
mod path;
mod video;

use clap::Parser;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use tracing::subscriber::SetGlobalDefaultError;
use transformrs::Provider;

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
struct Arguments {
    /// Path to the Typst input file.
    #[arg(long)]
    input: String,

    /// Verbose output.
    #[arg(long)]
    verbose: bool,

    /// Provider.
    ///
    /// Can be used to pass for example
    /// `--provider=openai-compatible(kokoros.transformrs.org)`.
    #[arg(long)]
    provider: Option<String>,

    /// Model.
    ///
    /// For the OpenAI compatible API from Kokoros, use `tts-1`.
    #[arg(long)]
    model: Option<String>,

    /// Voice.
    ///
    /// Note that DeepInfra at the time of writing supports more voices that
    /// Kokoros. If Kokoros respond with an empty file (which ffmpeg then
    /// crashes on), try a different voice.
    #[arg(long, default_value = "am_adam")]
    voice: String,

    /// Speed.
    ///
    /// Sets the speed of the voice. This is passed to the text-to-speech
    /// provider.
    #[arg(long)]
    speed: Option<f32>,

    /// Audio format.
    ///
    /// This setting usually should not be necessary since ffmpeg can handle
    /// most formats, but can be useful to override the default value.
    #[arg(long)]
    audio_format: Option<String>,

    /// Language code.
    ///
    /// This setting is required by Google.
    #[arg(long)]
    language_code: Option<String>,

    /// Out directory.
    #[arg(long, default_value = "_out")]
    out_dir: String,

    /// Enable caching.
    #[arg(long, default_value = "true")]
    cache: bool,

    /// Release.
    ///
    /// If true, attempt to convert the output video into a format that is more
    /// widely supported.
    #[arg(long, default_value = "false")]
    release: bool,

    /// Audio codec.
    ///
    /// This setting is passed to ffmpeg.
    ///
    /// Opus generally gives the best quality for the lowest file size, but is
    /// not supported by all platforms. For example, Whatsapp Web and X don't
    /// accept it.
    ///
    /// So therefore on MacOS set the value to `aac_at` and on Linux to
    /// `libfdk_aac`.
    #[arg(long, default_value = "opus")]
    audio_codec: String,
}

// TODO: This logic should be in the transformrs crate as `Provider::from_str`.
fn provider_from_str(s: &str) -> Provider {
    if s.starts_with("openai-compatible(") {
        let s = s.strip_prefix("openai-compatible(").unwrap();
        let s = s.strip_suffix(")").unwrap();
        let mut domain = s.to_string();
        if !domain.starts_with("https") {
            if domain.contains("localhost") {
                domain = format!("http://{}", domain);
            } else {
                domain = format!("https://{}", domain);
            }
        }
        Provider::OpenAICompatible(domain)
    } else if s == "google" {
        Provider::Google
    } else if s == "deepinfra" {
        Provider::DeepInfra
    } else {
        panic!("Unsupported provider: {}. Try setting a key like `GOOGLE_KEY` and not passing `--provider`.", s);
    }
}

/// Initialize logging with the given level.
fn init_subscriber(level: tracing::Level) -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stdout)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
}

/// Copy the input file to the output directory.
///
/// Typst requires the input to be present in the project directory.
fn copy_input(input: &str, dir: &str) -> PathBuf {
    let path = Path::new(dir).join("input.typ");
    std::fs::copy(input, &path).unwrap();
    path
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    if args.verbose {
        init_subscriber(tracing::Level::DEBUG).unwrap();
    } else {
        init_subscriber(tracing::Level::INFO).unwrap();
    }

    let dir = &args.out_dir;
    let path = Path::new(dir);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    let input = copy_input(&args.input, dir);

    let provider = args.provider.map(|p| provider_from_str(&p));
    let provider = provider.unwrap_or(Provider::DeepInfra);
    let mut other = HashMap::new();
    if provider != Provider::Google {
        other.insert("seed".to_string(), json!(42));
    }
    let config = transformrs::text_to_speech::TTSConfig {
        voice: Some(args.voice.clone()),
        output_format: args.audio_format.clone(),
        speed: args.speed,
        other: Some(other),
        language_code: args.language_code.clone(),
    };

    let slides = image::presenter_notes(&args.input);
    image::generate_images(&input, dir);
    let audio_ext = config.output_format.clone().unwrap_or("mp3".to_string());
    audio::generate_audio_files(
        &provider,
        dir,
        &slides,
        args.cache,
        &config,
        &args.model,
        &audio_ext,
    )
    .await;
    // Using mkv by default because it supports more audio formats.
    let output = "out.mkv";
    video::generate_video(dir, &slides, args.cache, &config, output, &audio_ext);
    if args.release {
        video::generate_release_video(dir, output, "release.mp4", &args.audio_codec);
    }
}
