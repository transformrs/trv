mod audio;
mod image;
mod video;

use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
use tracing::subscriber::SetGlobalDefaultError;

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
struct Arguments {
    /// Path to the Typst input file.
    #[arg(long)]
    input: String,

    /// Verbose output.
    #[arg(long)]
    verbose: bool,

    /// Out directory.
    #[arg(long, default_value = "_out")]
    out_dir: String,

    /// Enable caching.
    #[arg(long, default_value = "true")]
    cache: bool,
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

fn copy_input(input: &str, dir: &str) -> PathBuf {
    let path = Path::new(dir).join("input.pdf");
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

    image::generate_images(&input, dir);
    let slides = image::presenter_notes(&args.input);
    audio::generate_audio_files(dir, &slides, args.cache).await;
    video::create_video(dir, "out.mp4");
}
