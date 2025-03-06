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

fn core_html(out_dir: &str, slide: &Slide) -> String {
    let video_path = crate::path::video_path(out_dir, slide);
    let relative_path = video_path.strip_prefix(out_dir).unwrap().to_str().unwrap();
    format!(
        indoc::indoc! {"
        <h2>Slide {}</h2>

        <video controls>
          <source src='{}' type='video/mp4'>
          Your browser does not support the video tag.
        </video>
        "},
        slide.idx, relative_path
    )
}

fn index(args: &Arguments, slides: &[Slide], init: bool) -> String {
    let out_dir = &args.out_dir;
    let core = slides
        .iter()
        .map(|slide| core_html(out_dir, slide))
        .collect::<Vec<_>>()
        .join("\n");
    let waiting_text = if init {
        "Waiting for first build... Page will update when done."
    } else {
        ""
    };
    let release = if init {
        "".to_string()
    } else {
        format!(indoc::indoc! {"
            <h2>Release</h2>
            <video controls>
            <source src='release.mp4' type='video/mp4'>
            Your browser does not support the video tag.
            </video>
        "})
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
            <script>
                function getCurrentTimestamp() {{
                    const timestampElement = document.getElementById('timestamp');
                    return timestampElement ? timestampElement.innerText : null;
                }}

                function storeInitialTimestamp() {{
                    const currentTimestamp = getCurrentTimestamp();
                    if (currentTimestamp) {{
                        localStorage.setItem('pageTimestamp', currentTimestamp);
                    }}
                }}

                function checkTimestampAndReload() {{
                    const storedTimestamp = localStorage.getItem('pageTimestamp');
                    const currentTimestamp = getCurrentTimestamp();

                    if (storedTimestamp && currentTimestamp && storedTimestamp !== currentTimestamp) {{
                        setTimeout(() => {{}}, 800);
                        localStorage.setItem('pageTimestamp', currentTimestamp);

                        console.log('Reloading page');

                        // Force reload all videos by appending timestamp to source URLs
                        document.querySelectorAll('video').forEach(video => {{
                            const sources = video.querySelectorAll('source');
                            sources.forEach(source => {{
                                // Remove any existing timestamp parameter
                                let src = source.src.split('?')[0];
                                // Add new timestamp parameter
                                source.src = src + '?t=' + new Date().getTime();
                            }});
                            // Reload the video to apply the new sources
                            video.load();
                        }});

                        // Pause to let the live-server reload the files.
                        window.location.reload();
                    }}
                }}

                window.addEventListener('load', function() {{
                    setTimeout(() => {{}}, 200);

                    console.log('Adding random postfix to video sources');
                    // Add random postfix to video sources on initial load
                    document.querySelectorAll('video').forEach(video => {{
                        const sources = video.querySelectorAll('source');
                        sources.forEach(source => {{
                            // Remove any existing timestamp parameter
                            let src = source.src.split('?')[0];
                            // Add new random postfix
                            source.src = src + '?t=' + Math.random().toString(36).substring(2, 15);
                        }});
                        // Reload the video to apply the new source.
                        video.load();
                    }});
                }});
            </script>
        </head>
        <body>
            {}
            {}
            {}

            <div id='timestamp' style='display: none;'>{}</div>
        </body>
        </html>
        "},
        waiting_text, core, release, timestamp
    )
}

fn build_index(args: &Arguments, slides: &[Slide], init: bool) {
    let out_dir = &args.out_dir;
    let index = index(args, slides, init);
    let path = Path::new(out_dir).join("index.html");
    tracing::info!("Writing index.html");
    std::fs::write(path, index).unwrap();
}

async fn watch_build(input: PathBuf, args: &Arguments) {
    let slides = build(input.clone(), args).await;
    build_index(args, &slides, false);
}

fn spawn_server(args: &Arguments) {
    let out_dir = &args.out_dir;
    let root = format!("./{}", out_dir);

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

    let slides = [];
    build_index(args, &slides, true);
    spawn_server(args);
    watch_build(input.clone(), args).await;

    for result in &rx {
        match result {
            Ok(_event) => {
                watch_build(input.clone(), args).await;
                // Drain the channel to avoid processing old events.
                while let Ok(_) = rx.try_recv() {}
            }
            Err(e) => {
                tracing::debug!("watch error: {:?}", e);
            }
        }
    }
}
