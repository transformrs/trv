# trv

Transform slides and speaker notes into video.

[![Demo video](https://transformrs.github.io/trv/demo.png)](https://transformrs.github.io/trv/demo.mp4)

## Features

- 🔒 Fully offline generation of audio via the Kokoro text-to-speech model.
- 🛠️ Version control friendly - store your video source in git.
- 🚀 Caching of audio files to avoid redundant API calls.
- 🚀 Caching of video files for quick re-builds.
- 🚀 A development mode with a built-in web server for fast feedback.
- 🌐 Support for multiple languages and voices.
- 🚀 Small file sizes for easy sharing and hosting.

## Installation

```raw
$ cargo install trv
```

## Usage

This tool is designed to work with [Typst](https://github.com/typst/typst) presentations.
Typst is a new typesetting system that is similar to LaTeX.
To create a video, create a Typst presentation with speaker notes (we show only the first slide here):

```typ
#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")

// --- trv config:
// provider = "openai-compatible(kokoros.transformrs.org)"
// model = "tts-1"
// voice = "af_sky"
// speed = 0.95
// audio_format = "wav"
// ---

#slide[
    #set page(fill: black)
    #set text(fill: white)
    #v(6em)
    #set text(size: 35pt)
    #align(center)[*Text to video*]
    #toolbox.pdfpc.speaker-note(
    ```md
    What if you could easily generate videos from text?
    ```
    )
]
```

Next, we can work on the video with the following command:

```raw
$ trv watch examples/first.typ
```

This will start a local web server that will automatically update the video as you make changes to the presentation.

Once everything looks good, we can build the final video with the following command:

```raw
$ trv build examples/first.typ
```

This generates the following video:

[![First demo video](https://transformrs.github.io/trv/first.png)](https://transformrs.github.io/trv/first.mp4)

## Offline

To create a video without an API key nor an internet connection, you can self-host [Kokoros](https://github.com/lucasjinreal/Kokoros).
See the [Kokoros section](#kokoros) for more information.

## Via DeepInfra

For more voices and faster audio generation, you can use the Kokoro models hosted at DeepInfra.

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


To create a video without an API key nor an internet connection, you can self-host [Kokoros](https://github.com/lucasjinreal/Kokoros).
See the [Kokoros section](#kokoros) for more information.
Or for a state-of-the-art model with voice cloning capabilities, see the [Zyphra Zonos section](#zyphra-zonos).

## Kokoros

Kokoros is available at `kokoros.transformrs.org`, to use that one, set the following in your `trv` config:

```typ
// --- trv config:
// provider = "openai-compatible(kokoros.transformrs.org)"
// model = "tts-1"
// voice = "af_sky"
// audio_format = "wav"
// ---
```

If you want to use Kokoros locally, the easiest way is to use the Docker image.

```raw
$ git clone https://github.com/lucasjinreal/Kokoros.git

$ cd Kokoros/

$ docker build --rm -t kokoros .

$ docker run -it --rm -p 3000:3000 kokoros openai
```

Then, you can use the Docker image as the provider:

```typ
#import "@preview/polylux:0.4.0": *

// --- trv config:
// provider = "openai-compatible(localhost:3000)"
// model = "tts-1"
// voice = "af_sky"
// audio_format = "wav"
// ---

...
```

```raw
$ trv build presentation.typ
```

## Google

My favourite text-to-speech engine is the one from Google.

```raw
$ export GOOGLE_KEY="<YOUR KEY>"

$ trv build examples/google.typ
```

[![Google demo video](https://transformrs.github.io/trv/google.png)](https://transformrs.github.io/trv/google.mp4)

## Zyphra Zonos

To use the Zyphra Zonos model, you need 8 GB of VRAM.
So it's probably easiest to use DeepInfra:

```typst
#import "@preview/polylux:0.4.0": *

// --- trv config:
// provider = "deepinfra"
// model = "Zyphra/Zonos-v0.1-hybrid"
// voice = "american_male"
// ---
```

```raw
$ export DEEPINFRA_KEY="<YOUR KEY>"

$ trv build presentation.typ
```

Do note that Zonos is way more unstable than Kokoros at the time of writing.
For example, sometimes it will add random sounds like the sound of swallowing.
So in practice, the Kokoro model is probably the better option for now.

## Portrait Video

To create a portait video, like a YouTube Short, you can set the page to

```typ
#set page(width: 259.2pt, height: 460.8pt)
```

The rest should work as usual.
This will automatically create slides with 1080 x 1920 resolution since Typst is set to 300 DPI.
Next, ffmpeg will automatically scale the video to a height of 1920p so in this case the height will not be changed.
For landscape videos, it might scale the image down to 1920p.

## Subtitles

To add subtitles to the video, you can use OpenAI's [`whisper`](https://github.com/openai/whisper):

```raw
$ whisper _out/out.mp4 -f srt --model small --language=en
```

This will create a `out.srt` file with the subtitles.
