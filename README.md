# tv

Transform text and images into video.

## Installation

```sh
cargo install --git https://github.com/transformrs/trv
```

## Usage

Create a `.env` file with a `DEEPINFRA_KEY` variable

```
DEEPINFRA_KEY=<YOUR KEY>
```

Next, create a Typst presentation with speaker notes:

```typ
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
```

Now, to create a video from the presentation, run:

```sh
trv --input=presentation.typ
 INFO Generating audio file for slide 0
 INFO Generating audio file for slide 1
 INFO Creating video clip _out/1.mp4
 INFO Created video clip _out/1.mp4
 INFO Creating video clip _out/2.mp4
 INFO Created video clip _out/2.mp4
 INFO Concatenated video clips into _out/out.mp4
```

Now, the presentation is available as `_out/out.mp4`.
