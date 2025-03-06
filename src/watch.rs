use crate::build;
use crate::slide::Slide;
use crate::Arguments;
use live_server::listen;
use notify::recommended_watcher;
use notify::Event;
use notify::Result;
use notify::Watcher;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

fn add_timestamp(filename: &OsStr, timestamp: u64) -> String {
    let path = Path::new(filename);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let extension = path.extension().unwrap_or_default().to_str().unwrap_or("");
    format!("{}_{}.{}", stem, timestamp, extension)
}

fn core_html(out_dir: &str, slide: &Slide, timestamp: u64) -> String {
    let video_path = crate::path::video_path(out_dir, slide);
    let filename = video_path.file_name().unwrap();
    let filename = add_timestamp(filename, timestamp);
    format!(
        indoc::indoc! {"
        <h2>Slide {}</h2>

        <video controls>
          <source src='{}' type='video/mp4'>
          Your browser does not support the video tag.
        </video>
        "},
        slide.idx, filename
    )
}

fn index(args: &Arguments, slides: &[Slide], timestamp: u64, init: bool) -> String {
    let out_dir = &args.out_dir;
    let core = slides
        .iter()
        .map(|slide| core_html(out_dir, slide, timestamp))
        .collect::<Vec<_>>()
        .join("\n");
    let waiting_text = if init {
        "Waiting for first build... Page will update when done. This might take a while..."
    } else {
        ""
    };
    let release = if init {
        ""
    } else {
        indoc::indoc! {"
            <h2>Release</h2>
            <video controls>
            <source src='release.mp4' type='video/mp4'>
            Your browser does not support the video tag.
            </video>
        "}
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
                video {{
                    max-width: 800px;
                }}
            </style>
        </head>
        <body>
            {}
            {}
            {}

            <div id='timestamp' style='display: none;'>{}</div>
        </body>
        </html>
        "},
        waiting_text, core, timestamp, timestamp
    )
}

fn public_dir(args: &Arguments) -> PathBuf {
    let out_dir = &args.out_dir;
    let public_path = Path::new(out_dir).join("public");
    public_path
}

fn build_index(args: &Arguments, slides: &[Slide], timestamp: u64, init: bool) {
    let index = index(args, slides, timestamp, init);
    let path = public_dir(args).join("index.html");
    tracing::info!("Writing index.html.");
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

fn move_files_into_public(args: &Arguments, slides: &[Slide]) -> u64 {
    let public_path = public_dir(args);
    let out_dir = &args.out_dir;

    let timestamp = timestamp();

    for slide in slides {
        let video_path = crate::path::video_path(out_dir, slide);
        let filename = video_path.file_name().unwrap();
        let filename = add_timestamp(filename, timestamp);

        std::fs::copy(video_path, public_path.join(filename)).unwrap();
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
            if extension == "mp4" {
                let filename = path.file_name().unwrap().to_str().unwrap();
                if !filename.contains(&format!("_{}", timestamp)) {
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }
}

async fn watch_build(input: PathBuf, args: &Arguments) {
    let slides = build(input.clone(), args).await;
    let timestamp = move_files_into_public(args, &slides);
    build_index(args, &slides, timestamp, false);
    remove_old_files(args, timestamp);
}

fn spawn_server(args: &Arguments) {
    let root = format!("./{}", public_dir(args).display());

    let addr = "127.0.0.1:8080";
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

pub async fn watch(input: PathBuf, args: &Arguments) {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(&input, notify::RecursiveMode::NonRecursive)
        .expect("Failed to watch");

    let public_path = public_dir(args);
    if !public_path.exists() {
        std::fs::create_dir_all(&public_path).expect("Failed to create public directory");
    }

    let slides = [];
    let timestamp = timestamp();
    build_index(args, &slides, timestamp, true);
    spawn_server(args);
    watch_build(input.clone(), args).await;

    for result in &rx {
        match result {
            Ok(_event) => {
                watch_build(input.clone(), args).await;
                // Drain the channel to avoid processing old events.
                while rx.try_recv().is_ok() {}
            }
            Err(e) => {
                tracing::debug!("watch error: {:?}", e);
            }
        }
    }
}
