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
    #[arg(long, default_value = "hexgrad/Kokoro-82M")]
    model: String,

    /// Voice.
    ///
    /// Note that DeepInfra at the time of writing supports more voices that
    /// Kokoros. If Kokoros respond with an empty mp3 file (which ffmpeg then
    /// crashes on), try a different voice.
    #[arg(long, default_value = "am_adam")]
    voice: String,

    /// Audio format.
    ///
    /// This setting usually should not be necessary since ffmpeg can handle
    /// most formats, but can be useful to override the default value.
    #[arg(long, default_value = "mp3")]
    audio_format: String,

    /// Out directory.
    #[arg(long, default_value = "_out")]
    out_dir: String,

    /// Enable caching.
    #[arg(long, default_value = "true")]
    cache: bool,
}

// TODO: This logic should be in the transformrs crate as `Provider::from_str`.
fn provider_from_str(s: &str) -> Provider {
    if s.starts_with("openai-compatible(") {
        let s = s.strip_prefix("openai-compatible(").unwrap();
        let s = s.strip_suffix(")").unwrap();
        let mut domain = s.to_string();
        if !domain.starts_with("https://") {
            domain = format!("https://{}", domain);
        }
        return Provider::OpenAICompatible(domain);
    } else {
        panic!("Unsupported provider: {}. Try not passing `--provider`.", s);
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

    let mut other = HashMap::new();
    other.insert("seed".to_string(), json!(42));
    let config = transformrs::text_to_speech::TTSConfig {
        voice: Some(args.voice.clone()),
        output_format: Some(args.audio_format.clone()),
        speed: Some(1.25),
        other: Some(other),
        ..Default::default()
    };

    let provider = if let Some(provider) = args.provider {
        Some(provider_from_str(&provider))
    } else {
        None
    };

    let slides = image::presenter_notes(&args.input);
    image::generate_images(&input, dir);
    audio::generate_audio_files(&provider, dir, &slides, args.cache, &config, &args.model).await;
    video::generate_video(dir, &slides, &config, "out.mp4");
}
