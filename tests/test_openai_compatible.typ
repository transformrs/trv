#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// --- trv config:
// provider = "openai-compatible(kokoros.transformrs.org)"
// model = "tts-1"
// voice = "bm_lewis"
// audio_format = "wav"
// ---

#slide[
    \
    #align(center)[Code examples or code videos?]

    #toolbox.pdfpc.speaker-note(
        ```md
        ...... What if you could show code in a video?
        ```
    )
]

#slide[
    #set text(size: 20pt)

    ```rust
    #[tokio::main]
    async fn main() {
        println!("Hello, world!");
    }
    ```

    #toolbox.pdfpc.speaker-note(
        ```md
        For example, take this code.
        ```
    )
]
