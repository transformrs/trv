#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// --- trv config:
// provider = "elevenlabs"
// model = "eleven_turbo_v2_5"
// voice = "nPczCjzI2devNBz1zQrb"
// seed = 42
// ---

#slide[
    \
    #align(center)[Code examples or code videos?]

    #toolbox.pdfpc.speaker-note(
        ```md
        What if you could show code in a video?
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
