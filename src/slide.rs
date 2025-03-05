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

pub fn slides(input: &str) -> Vec<Slide> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slides() {
        let input = r#"
        #slide[
            first
            
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
}
