use crate::build;
use crate::slide::Slide;
use crate::Arguments;
use live_server::listen;
use notify::recommended_watcher;
use notify::Event;
use notify::Result;
use notify::Watcher;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

fn index(args: &Arguments, slides: &[Slide]) -> String {
    let out_dir = &args.out_dir;
    let core = slides
        .iter()
        .map(|slide| {
            let video_path = crate::path::video_path(out_dir, slide);
            let relative_path = video_path.strip_prefix(out_dir).unwrap().to_str().unwrap();
            format!("<h2>Slide {}</h2><a href=\"{}\">{}</a>", slide.idx, relative_path, relative_path)
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        indoc::indoc! {"<!DOCTYPE html>
        <html>
        <head>
            <title>trv</title>
        </head>
        <body>
            {}
        </body>
        </html>
        "},
        core
    )
}

fn build_index(args: &Arguments, slides: &[Slide]) {
    let out_dir = &args.out_dir;
    let index = index(args, slides);
    let path = Path::new(out_dir).join("index.html");
    tracing::info!("Writing index.html");
    std::fs::write(path, index).unwrap();
}

async fn watch_build(input: PathBuf, args: &Arguments) {
    let slides = build(input.clone(), args).await;
    build_index(args, &slides);
}

fn spawn_server(args: &Arguments) {
    let out_dir = &args.out_dir;
    let root = format!("./{}", out_dir);

    let addr = "127.0.0.1:8080";
    tracing::info!("Starting server at http://{}", addr);
    let options = live_server::Options::default();

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

    let slides = [];
    build_index(args, &slides);
    spawn_server(args);
    watch_build(input.clone(), args).await;

    for result in rx {
        match result {
            Ok(_event) => {
                watch_build(input.clone(), args).await;
            }
            Err(e) => {
                tracing::debug!("watch error: {:?}", e);
            }
        }
    }
}
