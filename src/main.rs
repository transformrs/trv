use clap::Parser;
use tracing::subscriber::SetGlobalDefaultError;
use transformrs::Key;

#[derive(clap::Subcommand)]
enum Commands {
}

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Multimodal AI command line interface"
)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
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

fn find_single_key(keys: transformrs::Keys) -> Key {
    let keys = keys.keys;
    if keys.len() != 1 {
        eprintln!("Expected exactly one key, found {}", keys.len());
        std::process::exit(1);
    }
    keys[0].clone()
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

    println!("Hello, world!");
}
