mod audio;
mod image;
mod path;
mod slide;
mod video;

use clap::Parser;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use tracing::subscriber::SetGlobalDefaultError;
use transformrs::Provider;

#[derive(Clone, Debug, Default, Deserialize)]
struct Config {
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
    pub speed: Option<f32>,

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
struct BuildArgs {
    /// Path to the Typst input file.
    input: PathBuf,
}

#[derive(Clone, Debug, Parser)]
struct WatchArgs {
    /// Path to the Typst input file.
    input: PathBuf,
}

#[derive(Clone, Debug, clap::Subcommand)]
enum Task {
    /// Build the video.
    Build(BuildArgs),

    /// Watch the input file and rebuild the video when it changes.
    Watch(WatchArgs),
}

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
struct Arguments {
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

fn include_includes(input_dir: &Path, content: &str) -> String {
    let mut output = String::new();
    for line in content.lines() {
        if line.starts_with("#include") {
            let include = line.split_whitespace().nth(1).unwrap().trim_matches('"');
            let include_path = input_dir.join(include);
            tracing::debug!("Including file: {}", include_path.display());
            let content = std::fs::read_to_string(include_path).unwrap();
            for line in content.lines() {
                output.push_str(line);
                output.push('\n');
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }
    output
}

/// Copy the Typst input file to the output directory.
///
/// This is necessary because Typst requires the input to be present in the
/// project directory.
fn copy_input_with_includes(dir: &str, input: &PathBuf) -> PathBuf {
    let output_path = Path::new(dir).join("input.typ");
    let content = std::fs::read_to_string(input).unwrap();
    let input_dir = Path::new(input).parent().unwrap();
    let content = include_includes(input_dir, &content);
    std::fs::write(&output_path, content).unwrap();

    output_path
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
    let input = match args.task {
        Task::Build(args) => args.input,
        Task::Watch(args) => args.input,
    };
    let copied_input = copy_input_with_includes(dir, &input);
    let config = parse_config(&copied_input);

    let provider = config.provider.map(|p| provider_from_str(&p));
    let provider = provider.unwrap_or(Provider::DeepInfra);
    let mut other = HashMap::new();
    if provider != Provider::Google {
        other.insert("seed".to_string(), json!(42));
    }
    let tts_config = transformrs::text_to_speech::TTSConfig {
        voice: Some(config.voice.clone()),
        output_format: config.audio_format.clone(),
        speed: config.speed,
        other: Some(other),
        language_code: config.language_code.clone(),
    };

    let slides = slide::slides(copied_input.to_str().unwrap());
    if slides.is_empty() {
        panic!("No slides found in input file: {}", input.display());
    }
    image::generate_images(&copied_input, dir);
    let audio_ext = tts_config
        .output_format
        .clone()
        .unwrap_or("mp3".to_string());
    audio::generate_audio_files(
        &provider,
        dir,
        &slides,
        args.cache,
        &tts_config,
        &config.model,
        &audio_ext,
    )
    .await;
    // Using mkv by default because it supports more audio formats.
    let output = "out.mkv";
    video::generate_video(dir, &slides, args.cache, &tts_config, output, &audio_ext);
    if args.release {
        video::generate_release_video(dir, output, "release.mp4", &args.audio_codec);
    }
}
