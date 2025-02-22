# trv

Transform slides and speaker notes into video.

## Installation

```sh
cargo install trv
```

Or with [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall):

```sh
cargo binstall trv
```

## Usage

Create a Typst presentation with speaker notes:

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

To create a video without an API key nor an internet connection, you can self-host [Kokoros](https://github.com/lucasjinreal/Kokoros).
See the [Kokoros section](#kokoros) for more information.

A simple alternative is to use the hosted version at <https://kokoros.transformrs.org>.
For example, this command creates a video using the hosted service:

```raw
$ trv --input=presentation.typ \
    --provider=openai-compatible(kokoros.transformrs.org) \
    --model=tts-1 \
    --voice=bm_lewis \
    --audio-format=wav \
    --release
```

To create a video from the presentation with DeepInfra, run:

```raw
$ export DEEPINFRA_KEY="<YOUR KEY>"

$ trv --input=presentation.typ
 INFO Generating audio file for slide 0
 INFO Generating audio file for slide 1
 INFO Creating video clip _out/1.mp4
 INFO Created video clip _out/1.mp4
 INFO Creating video clip _out/2.mp4
 INFO Created video clip _out/2.mp4
 INFO Concatenated video clips into _out/out.mp4
```

Now, the presentation is available as `_out/out.mp4`.
A benefit of DeepInfra is that they have some extra voices compared to Kokoros.

## Kokoros

To use Kokoros locally, the easiest way is to use the Docker image.

```sh
$ git clone https://github.com/lucasjinreal/Kokoros.git

$ cd Kokoros/

$ docker build --rm -t kokoros .

$ docker run -it --rm -p 3000:3000 kokoros openai
```

Then, you can use the Docker image as the provider:

```raw
$ trv --input=presentation.typ --provider=openai-compatible(localhost:3000)
```

## Compatibility

The output video is not compatible with all video players.
To make it more compatible, you can use `ffmpeg`.
For example, this script converts the video to a more compatible format with aac audio.

```sh
#!/usr/bin/env bash

set -e

# Convert video to highly-compatible format with aac audio.
# Using the advanced aac encoder via fdk-aac (unix) or aac_at (apple).

ffmpeg \
    -y \
    -i _out/out.mp4 \
    -c:v libx264 -crf 23 -preset fast \
    -vf "scale=-1:1080,format=yuv420p" \
    -c:a aac_at \
    -strict experimental \
    _out/out-aac.mp4
```

## About Audio

Audio is generated using the [transformrs](https://github.com/transformrs/transformrs) crate.
It supports multiple providers, including DeepInfra, OpenAI, and Google.

So `trv` should also work with providers other than DeepInfra.
However, during testing, I got the best results with DeepInfra for the lowest price.

For example, OpenAI text-to-speech requires any video to contain a "clear disclosure" that the voice they are hearing is AI-generated.

Google, meanwhile, has the best text-to-speech engine that I've found as part of Gemini 2.0 Flash Experimental.
However, audio output is not yet available via the API.
