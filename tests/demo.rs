use std::fs;
use std::path::Path;

fn find_code_block_before(lines: &[&str], start_index: usize) -> String {
    let mut code_block = String::new();
    let mut i = start_index;
    while i > 0 {
        i -= 1;
        if lines[i].starts_with("```") {
            // Skip the opening ```raw or ```
            i += 1;
            while i < lines.len() && !lines[i].starts_with("```") {
                code_block.push_str(lines[i].trim());
                code_block.push('\n');
                i += 1;
            }
            break;
        }
    }
    code_block
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

#[test]
fn test_readme_video_links() {
    let readme = fs::read_to_string("README.md").unwrap();
    let lines: Vec<&str> = readme.lines().collect();

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

            // Find and clean the previous code block
            let code_block = find_code_block_before(&lines, i);
            let clean_code_block = clean_content(&code_block);

            // Read and clean the actual script
            let script_content = fs::read_to_string(&script_name).unwrap();
            let clean_script = clean_content(&script_content);
            if clean_code_block != clean_script {
                println!("\nExpected script content:\n{}\n", clean_code_block);
                println!("Actual script content:\n{}\n", clean_script);
                panic!(
                    "Script {} content doesn't match README code block",
                    script_name
                );
            }
        }
    }
}

