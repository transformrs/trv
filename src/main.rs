mod audio;
mod image;
mod path;
mod slide;
mod video;
mod watch;

use crate::slide::Slide;
use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::subscriber::SetGlobalDefaultError;
use transformrs::Provider;
use watch::watch;

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct Config {
    /// Provider.
    ///
    /// Can be used to pass for example
    /// `--provider=openai-compatible(kokoros.transformrs.org)`.
    pub provider: Option<String>,

    /// Text-to-speech model.
    ///
    /// For the OpenAI compatible API from Kokoros, use `tts-1`.
    pub model: Option<String>,

    /// Text-to-speech voice.
    ///
    /// Note that DeepInfra at the time of writing supports more voices that
    /// Kokoros. If Kokoros respond with an empty file (which ffmpeg then
    /// crashes on), try a different voice.
    pub voice: String,

    /// Audio format.
    ///
    /// This setting usually should not be necessary since ffmpeg can handle
    /// most formats, but can be useful to override the default value.
    pub audio_format: Option<String>,

    /// Text-to-speech voice speed.
    ///
    /// Sets the speed of the voice. This is passed to the text-to-speech
    /// provider.
    pub speed: Option<f64>,

    /// Text-to-speech seed.
    pub seed: Option<u64>,

    /// Text-to-speech language code.
    ///
    /// This setting is required by Google.
    pub language_code: Option<String>,
}

/// Parse the config from the Typst input file.
fn parse_config(input: &PathBuf) -> Config {
    let content = std::fs::read_to_string(input).unwrap();
    let lines = content.lines();
    let mut config_lines = Vec::new();
    let mut in_config_section = false;

    for line in lines {
        let line = line.trim();
        if line == "// --- trv config:" {
            in_config_section = true;
            continue;
        } else if in_config_section && line == "// ---" {
            break;
        } else if in_config_section {
            let stripped = line
                .strip_prefix("// ")
                .expect("line is not prefixed with //");
            config_lines.push(stripped);
        }
    }

    let config_str = config_lines.join("\n");
    if config_str.is_empty() {
        return Config::default();
    }
    let config: Config = toml::from_str(&config_str).unwrap();
    config
}

#[test]
fn test_parse_config() {
    let config = parse_config(&PathBuf::from("tests/test_openai_compatible.typ"));
    assert_eq!(config.audio_format, Some("wav".to_string()));
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct BuildArgs {
    /// Path to the Typst input file.
    input: PathBuf,

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

#[derive(Clone, Debug, Parser)]
pub(crate) struct WatchArgs {
    /// Path to the Typst input file.
    input: PathBuf,

    /// Port to run the server on.
    #[arg(long, default_value = "8080")]
    port: u16,

    /// Command to run before compiling the Typst file.
    #[arg(long)]
    pre_typst: Option<String>,
}

#[derive(Clone, Debug, clap::Subcommand)]
enum Task {
    /// Build the video.
    Build(BuildArgs),

    /// Watch the current directory and rebuild the video on change.
    Watch(WatchArgs),
}

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
pub(crate) struct Arguments {
    #[command(subcommand)]
    task: Task,

    /// Verbose output.
    #[arg(long)]
    verbose: bool,

    /// Out directory.
    #[arg(long, default_value = "_out")]
    out_dir: String,

    /// Enable caching.
    #[arg(long, default_value = "true")]
    cache: Option<bool>,
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

pub(crate) fn audio_format(config: &Config) -> String {
    config.audio_format.clone().unwrap_or("mp3".to_string())
}

pub(crate) async fn build(
    input: PathBuf,
    config: &Config,
    args: &Arguments,
    release: bool,
    audio_codec: Option<String>,
) -> Vec<Slide> {
    let out_dir = &args.out_dir;

    let provider = config
        .provider
        .as_ref()
        .map(|p| Provider::from_str(p).unwrap());
    let provider = provider.unwrap_or(Provider::DeepInfra);

    let slides = slide::slides(input.to_str().unwrap());
    if slides.is_empty() {
        panic!("No slides found in input file: {}", input.display());
    }
    image::generate_images(&input, out_dir);
    let audio_ext = audio_format(config);
    let cache = args.cache.unwrap();
    audio::generate_audio_files(&provider, out_dir, &slides, cache, config, &audio_ext).await;
    let output = "out.mp4";
    if release {
        let audio_codec = audio_codec.unwrap();
        video::combine_video(out_dir, &slides, config, &provider, output, &audio_codec);
    }
    slides
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

    match args.task {
        Task::Build(ref build_args) => {
            let release = true;
            let audio_codec = Some(build_args.audio_codec.clone());
            let config = parse_config(&build_args.input);
            let _ = build(
                build_args.input.clone(),
                &config,
                &args,
                release,
                audio_codec,
            )
            .await;
        }
        Task::Watch(ref watch_args) => {
            let config = parse_config(&watch_args.input);
            watch(watch_args, &config, &args).await
        }
    };
}
