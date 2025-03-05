use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Slide {
    pub idx: u64,
    pub content: String,
    pub speaker_note: String,
}

fn speaker_note(content: &str) -> String {
    let regions = find_regions(content, r#"pdfpc.speaker-note"#, Symbol::Parenthesis);
    if regions.len() == 1 {
        regions[0]
            .trim_start_matches("pdfpc.speaker-note(")
            .trim()
            .trim_end_matches(")")
            .trim()
            .trim_matches('"')
            .trim_start_matches("```md")
            .trim_end_matches("```")
            .trim()
            .to_string()
    } else if regions.is_empty() {
        "".to_string()
    } else {
        panic!("Expected 0 or 1 regions, got {}", regions.len());
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

fn find_regions(content: &str, start_pattern: &str, symbol: Symbol) -> Vec<String> {
    content
        .match_indices(start_pattern)
        .map(|(idx, _)| {
            let start = idx;
            let end = find_end(content, start, symbol);
            content[start..end].to_string()
        })
        .collect()
}

#[test]
fn test_find_end() {
    let content = r#"
    #slide[
        #align(center)[#align(right)[first]]
        foo
    ]
    #slide[
        two
    ]
    bar
    "#;
    let regions = find_regions(content, "#slide[", Symbol::SquareBracket);
    assert_eq!(regions.len(), 2);
    assert!(regions[0].starts_with("#slide["));
    assert!(regions[0].ends_with("]"));
    assert!(regions[0].contains("right"));
    assert!(!regions[0].contains("two"));
    assert!(!regions[0].contains("bar"));
    assert!(regions[0].contains("foo"));
    assert!(regions[1].contains("two"));
    assert!(!regions[1].contains("bar"));
}

fn slides(input: &str) -> Vec<Slide> {
    find_regions(input, "#slide[", Symbol::SquareBracket)
        .iter()
        .enumerate()
        .map(|(idx, content)| {
            let speaker_note = speaker_note(content);
            let idx = (idx + 1) as u64;
            Slide {
                idx,
                content: content.to_string(),
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
