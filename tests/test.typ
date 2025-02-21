#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")

#set text(size: 25pt)

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    What if you could show code in a video?
    ```
    )

    \
    #align(center)[Code examples or code videos?]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    For example, take this code.
    ```
    )

    #set text(size: 20pt)

    ```rust
    #[tokio::main]
    async fn main() {
        println!("Hello, world!");
    }
    ```
]

