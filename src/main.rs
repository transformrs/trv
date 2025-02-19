use clap::Parser;
use std::io::Read;
use tracing::subscriber::SetGlobalDefaultError;

#[derive(clap::Subcommand)]
enum Commands {}

#[derive(Parser)]
#[command(author, version, about = "Text and image to video")]
struct Arguments {
    /// Path to the input file.
    ///
    /// If not provided, reads from stdin.
    #[arg(short, long)]
    input: Option<String>,

    /// Verbose output.
    ///
    /// The output of the logs is printed to stderr because the output is
    /// printed to stdout.
    #[arg(long)]
    verbose: bool,
}

pub enum Task {
    #[allow(clippy::upper_case_acronyms)]
    TTS,
}

/// Initialize logging with the given level.
fn init_subscriber(level: tracing::Level) -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level)
        .with_writer(std::io::stderr)
        .without_time()
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    if args.verbose {
        init_subscriber(tracing::Level::DEBUG).unwrap();
    } else {
        init_subscriber(tracing::Level::INFO).unwrap();
    }

    let input = if let Some(input) = args.input {
        std::fs::read_to_string(&input).expect("Failed to read from file")
    } else {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    println!("input: {}", input);
}
