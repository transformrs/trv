use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn find_text_before(lines: &[&str], start_index: usize) -> String {
    let mut text = String::new();
    let mut i = start_index;
    while i > 0 {
        i -= 1;
        let line = lines[i].trim();
        if line.is_empty() {
            continue;
        }
        // Stop when we hit a heading or another link
        if line.starts_with('#') || (line.contains("](") && line.contains(".mp4")) {
            break;
        }
        // Add non-empty lines to the beginning of our text
        if !line.is_empty() {
            text = format!("{}\n{}", line, text);
        }
    }
    text.trim().to_string()
}

fn extract_code_block(content: &str) -> String {
    let mut code_block = String::new();
    let mut in_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        }
    }
    code_block
}

fn drop_export_line(content: &str) -> String {
    content
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("$ export")
                && !trimmed.starts_with("export")
                && !trimmed.starts_with("cp")
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

fn clean_content(content: &str) -> String {
    content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        .collect::<Vec<&str>>()
        .join("\n")
        .trim()
        .to_string()
}

fn get_script_name(video_link: &str) -> String {
    let video_name = video_link
        .split('/')
        .last()
        .unwrap()
        .trim_end_matches(')')
        .trim_end_matches(".mp4");

    format!("examples/{}.sh", video_name)
}

fn remove_spaces_prefix(text: &str) -> String {
    text.lines()
        .map(|l| l.trim_start())
        .collect::<Vec<&str>>()
        .join("\n")
}

#[derive(Clone, Debug)]
struct LinkAndCode {
    pub script_name: String,
    pub clean_text: String,
}

fn extract_readme_links_and_code() -> HashMap<String, LinkAndCode> {
    let readme = std::fs::read_to_string("README.md").unwrap();
    let lines: Vec<&str> = readme.lines().collect();
    let mut links_and_code = std::collections::HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        // Look for video links ending in .mp4
        if line.contains(".mp4") && line.contains("](") {
            let script_name = get_script_name(line);

            // Find and clean the text before the video link
            let text_before = find_text_before(&lines, i);
            let clean_text = extract_code_block(&text_before);
            let clean_text = clean_content(&clean_text);
            let clean_text = drop_export_line(&clean_text);
            let clean_text = clean_text
                .strip_prefix("$ ")
                .unwrap_or(&clean_text)
                .to_string();

            let link_and_code = LinkAndCode {
                script_name: script_name.clone(),
                clean_text,
            };
            links_and_code.insert(script_name, link_and_code);
        }
    }

    links_and_code
}

#[test]
fn test_readme_video_links() {
    let readme = fs::read_to_string("README.md").unwrap();
    let lines: Vec<&str> = readme.lines().collect();

    let links_and_code = extract_readme_links_and_code();
    for (script_name, link_and_code) in links_and_code {
        assert!(
            link_and_code.clean_text.starts_with("trv"),
            "Expected script for {} to start with 'trv', but got:\n{}",
            script_name,
            link_and_code.clean_text
        );
    }

    for (i, line) in lines.iter().enumerate() {
        // Look for video links ending in .mp4
        if line.contains(".mp4") && line.contains("](") {
            let script_name = get_script_name(line);
            println!("script_name: {}", script_name);

            // Check script exists
            assert!(
                Path::new(&script_name).exists(),
                "Expected script {} to exist for video link in line: {}",
                script_name,
                line
            );

            // Find and clean the text before the video link
            let text_before = find_text_before(&lines, i);
            let clean_text = extract_code_block(&text_before);
            let clean_text = clean_content(&clean_text);
            let clean_text = drop_export_line(&clean_text);
            let clean_text = clean_text.strip_prefix("$ ").unwrap_or(&clean_text);

            // Read and clean the actual script
            let script_content = fs::read_to_string(&script_name).unwrap();
            let clean_script = clean_content(&script_content);
            let clean_script = drop_export_line(&clean_script);
            let clean_script = remove_spaces_prefix(&clean_script);

            if clean_text != clean_script {
                println!("\nREADME code:\n{}\n", clean_text);
                println!("Script code:\n{}\n", clean_script);
                panic!(
                    "Script {} content doesn't match README text before link",
                    script_name
                );
            }
        }
    }
}
