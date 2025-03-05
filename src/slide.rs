use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Slide {
    pub idx: u64,
    pub content: String,
    pub speaker_note: String,
}

fn speaker_note(content: &str) -> String {
    println!("content: {content:?}");
    let rx = Regex::new(r"pdfpc\.speaker-note\(([^\)]*)\)").unwrap();
    if let Some(cap) = rx.captures(content) {
        cap[1]
            .trim()
            .trim_matches('"')
            .trim_start_matches("```md")
            .trim_end_matches("```")
            .trim()
            .to_string()
    } else {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy)]
enum Symbol {
    SquareBracket,
    Parenthesis,
}

fn find_end(content: &str, start: usize, symbol: Symbol) -> usize {
    let mut depth = 0;
    let chars = content.chars().skip(start).collect::<Vec<_>>();
    let start_char = match symbol {
        Symbol::SquareBracket => '[',
        Symbol::Parenthesis => '(',
    };
    let mut has_started = false;
    let end_char = match symbol {
        Symbol::SquareBracket => ']',
        Symbol::Parenthesis => ')',
    };
    for (i, c) in chars.iter().enumerate() {
        if c == &start_char {
            depth += 1;
            has_started = true;
        } else if c == &end_char {
            depth -= 1;
        }
        if depth == 0 && has_started {
            return i + start + 1;
        }
    }
    panic!("No end found");
}

#[test]
fn test_find_end() {
    let content = r#"
    #slide[
        #align(center)[#align(right)[foo]]
        foo
    ]
    bar
    "#;
    let starts = content
        .match_indices("#slide[")
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    let start = starts[0];
    let end = find_end(content, start, Symbol::SquareBracket);
    let result = content[start..end].to_string();
    assert!(result.starts_with("#slide["));
    assert!(result.ends_with("]"));
}

fn slides(input: &str) -> Vec<Slide> {
    let slide = Regex::new(r"#slide\[[^\]]*\]").unwrap();
    slide
        .captures_iter(input)
        .enumerate()
        .map(|(idx, cap)| {
            let content = cap[0].to_string();
            let speaker_note = speaker_note(&content);
            let idx = (idx + 1) as u64;
            Slide {
                idx,
                content,
                speaker_note,
            }
        })
        .collect()
}

pub fn slides_from_file(input: &str) -> Vec<Slide> {
    let input = std::fs::read_to_string(input).unwrap();
    slides(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slides() {
        let input = r#"
        #slide[
            #align(center)[first]
            
            #toolbox.pdfpc.speaker-note("
            first note
            ")
        ]

        #slide[
            #v(8em)
            second

            #toolbox.pdfpc.speaker-note(
            ```md
            second note
            ```
            )
        ]
        "#;
        let slides = slides(input);
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].idx, 1);
        assert!(slides[0].content.contains("first"));
        assert_eq!(slides[0].speaker_note, "first note");
        assert_eq!(slides[1].idx, 2);
        assert!(slides[1].content.contains("second"));
        assert_eq!(slides[1].speaker_note, "second note");
    }

    #[test]
    fn test_slides_from_file() {
        let input = "tests/test.typ";
        let slides = slides_from_file(input);
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].idx, 1);
        assert!(slides[0].content.contains("Code examples or code videos?"));
        assert!(slides[0]
            .speaker_note
            .contains("What if you could show code in a video?"));
    }
}
