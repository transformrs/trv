use crate::path::PathStr;
use std::path::Path;
use std::path::PathBuf;

pub fn generate_images(input: &PathBuf, dir: &str) {
    let image_dir = Path::new(dir).join("image");
    if !image_dir.exists() {
        std::fs::create_dir_all(&image_dir).unwrap();
    }
    let image_dir = image_dir.to_string();
    let output = std::process::Command::new("typst")
        .arg("compile")
        .arg("--format=png")
        .arg("--ppi=300")
        .arg(input)
        .arg(format!("{image_dir}/{{p}}.png"))
        .output()
        .expect("Failed to run typst compile");

    if !output.status.success() {
        eprintln!("Error running typst compile:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    if !output.stdout.is_empty() {
        tracing::info!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
