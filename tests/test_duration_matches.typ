#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// --- trv config:
// provider = "google"
// voice = "en-US-Chirp-HD-D"
// language_code = "en-US"
// ---

#slide[
    \
    #align(center)[Code examples or code videos?]

    #toolbox.pdfpc.speaker-note("
        What if you could show code in a video?
    ")
]

#slide[
    #set text(size: 20pt)

    ```rust
    #[tokio::main]
    async fn main() {
        println!("Hello, world!");
    }
    ```

    #toolbox.pdfpc.speaker-note("
        For example, take this code.
        With some more text to make the video duration longer.
    ")
]

#slide[
    three

    #toolbox.pdfpc.speaker-note("
        Since the longer the video the higher the chance that the audio and video will be out of sync,
    ")
]

#slide[
    four

    #toolbox.pdfpc.speaker-note("
        To be sure, here is some more audio.
    ")
]

#slide[
    five

    #toolbox.pdfpc.speaker-note("
        And here is even more audio.
    ")
]
