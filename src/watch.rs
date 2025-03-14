use crate::audio_format;
use crate::build;
use crate::slide::Slide;
use crate::Arguments;
use crate::Config;
use crate::WatchArgs;
use ignore::Walk;
use live_server::listen;
use notify::recommended_watcher;
use notify::Event;
use notify::Result;
use notify::Watcher;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

/// Add a timestamp to the filename.
///
/// This is used to bust the browser cache (force update).
fn add_timestamp(filename: &OsStr, timestamp: u64) -> String {
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap_or("");
    format!("{}_{}.{}", stem, timestamp, extension)
}

fn core_html(out_dir: &str, slide: &Slide, timestamp: u64, config: &Config) -> String {
    let image_path = crate::path::image_path(out_dir, slide);
    let image_file = image_path.file_name().unwrap();
    let image_file = add_timestamp(image_file, timestamp);
    let audio_ext = audio_format(config);
    let audio_path = crate::path::audio_path(out_dir, slide, &audio_ext);
    let audio_file = audio_path.file_name().unwrap();
    let audio_file = add_timestamp(audio_file, timestamp);
    format!(
        indoc::indoc! {"
        <div class='slide'>
            <h2>Slide {}</h2>

            <a href='{}'>
                <img src='{}' alt='Slide {}'/><br/>
            </a>
            <audio controls src='{}'/></audio>
        </div>
        "},
        slide.idx, image_file, image_file, slide.idx, audio_file
    )
}

fn index(
    args: &Arguments,
    config: &Config,
    slides: &[Slide],
    timestamp: u64,
    init: bool,
) -> String {
    let out_dir = &args.out_dir;
    let core = slides
        .iter()
        .map(|slide| core_html(out_dir, slide, timestamp, config))
        .collect::<Vec<_>>()
        .join("\n");
    let waiting_text = if init {
        "Waiting for first build... Page will update when done. This might take a while..."
    } else {
        ""
    };
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!(
        indoc::indoc! {"<!DOCTYPE html>
        <html lang='en'>
        <head>
            <title>trv</title>
            <meta charset='UTF-8'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <style>
                body {{
                    text-align: center;
                }}
                img {{
                    max-width: 800px;
                    max-height: 80vh;
                    border: 1px solid black;
                }}
                audio {{
                    width: 800px;
                }}
                .slide {{
                    margin-bottom: 60px;
                }}
                .slide h2 {{
                    margin-bottom: 10px;
                }}
            </style>
        </head>
        <body>
            {}
            {}

            <div id='timestamp' style='display: none;'>{}</div>
        </body>
        </html>
        "},
        waiting_text, core, timestamp
    )
}

fn public_dir(args: &Arguments) -> PathBuf {
    let out_dir = &args.out_dir;
    let public_path = Path::new(out_dir).join("public");
    public_path
}

fn build_index(args: &Arguments, config: &Config, slides: &[Slide], timestamp: u64, init: bool) {
    let index = index(args, config, slides, timestamp, init);
    let path = public_dir(args).join("index.html");
    tracing::info!("Writing index.html");
    std::fs::write(path, index).unwrap();
}

/// Timestamp in unix seconds.
///
/// Used for cache busting.
fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn move_files_into_public(args: &Arguments, config: &Config, slides: &[Slide]) -> u64 {
    let public_path = public_dir(args);
    let out_dir = &args.out_dir;

    let timestamp = timestamp();

    for slide in slides {
        let image_path = crate::path::image_path(out_dir, slide);
        let filename = image_path.file_name().unwrap();
        let filename = add_timestamp(filename, timestamp);

        std::fs::copy(image_path, public_path.join(filename)).unwrap();

        let audio_ext = audio_format(config);
        let audio_path = crate::path::audio_path(out_dir, slide, &audio_ext);
        let filename = audio_path.file_name().unwrap();
        let filename = add_timestamp(filename, timestamp);
        std::fs::copy(audio_path, public_path.join(filename)).unwrap();
    }
    timestamp
}

fn remove_old_files(args: &Arguments, timestamp: u64) {
    let public_path = public_dir(args);
    for entry in std::fs::read_dir(public_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(extension) = path.extension() {
            if extension != "html" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                if !filename.contains(&format!("_{}", timestamp)) {
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Status of the command.
///
/// This can be used to avoid crashing the watch loop completely. Instead,
/// report an error and ignore further actions until the loop is called again.
/// This allows the user to fix the problem and continue without having to
/// manually restart the `trv watch`.
enum Status {
    Success,
    Failure,
}

fn run_pre_typst(watch_args: &WatchArgs) -> Status {
    if let Some(pre_typst) = &watch_args.pre_typst {
        tracing::info!("Running pre-typst command...");
        let mut cmd = std::process::Command::new("/usr/bin/env");
        cmd.arg("bash");
        cmd.arg("-c");
        cmd.arg(pre_typst);
        let output = cmd.output().expect("Failed to run pre-typst command");
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("pre-typst command failed: {}", stderr.trim());
            return Status::Failure;
        }
    }
    Status::Success
}

async fn watch_build(watch_args: &WatchArgs, config: &Config, args: &Arguments) {
    let release = false;
    let input = watch_args.input.clone();
    let audio_codec = None;

    let status = run_pre_typst(watch_args);
    if status == Status::Success {
        let slides = build(input.clone(), config, args, release, audio_codec).await;
        let timestamp = move_files_into_public(args, config, &slides);
        build_index(args, config, &slides, timestamp, false);
        remove_old_files(args, timestamp);
    }
}

fn spawn_server(watch_args: &WatchArgs, args: &Arguments) {
    let root = format!("./{}", public_dir(args).display());
    let addr = format!("127.0.0.1:{}", watch_args.port);
    tracing::info!("Starting server at http://{}", addr);
    let options = live_server::Options {
        // In Chrome, hard reloads are required to see video previews.
        hard_reload: true,
        ..Default::default()
    };

    tokio::spawn(async move {
        listen(addr, root)
            .await
            .unwrap()
            .start(options)
            .await
            .unwrap();
    });
}

pub async fn watch(watch_args: &WatchArgs, config: &Config, args: &Arguments) {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx).unwrap();
    let mode = notify::RecursiveMode::NonRecursive;
    // Watch the current directory since that is probably the most intuitive
    // path to watch. It also would allow watching scripts that are in a
    // directory that is above the Typst file. For Typst, files have to be in
    // the same directory, but allowing the current directory gives more
    // flexibility.
    //
    // Flatten ignores the errors (e.g., permission errors).
    for entry in Walk::new("./").flatten() {
        let path = entry.path();
        if !path.is_dir() {
            watcher.watch(path, mode).expect("Failed to watch");
        }
    }

    let public_path = public_dir(args);
    if !public_path.exists() {
        std::fs::create_dir_all(&public_path).expect("Failed to create public directory");
    }

    let slides = [];
    let timestamp = timestamp();
    build_index(args, config, &slides, timestamp, true);
    spawn_server(watch_args, args);
    watch_build(watch_args, config, args).await;

    for result in &rx {
        match result {
            Ok(_event) => {
                watch_build(watch_args, config, args).await;
                // Drain the channel to avoid processing old events.
                while rx.try_recv().is_ok() {}
            }
            Err(e) => {
                tracing::debug!("watch error: {:?}", e);
            }
        }
    }
}
