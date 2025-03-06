#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// This files assumes that the DEEPINFRA_KEY environment variable is set.
// With that, trv will automatically use the DeepInfra provider.

// --- trv config:
// audio_format = "mp3"
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
