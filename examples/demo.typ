#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set page(fill: black, margin: 2em)
#set text(fill: white)
#set text(size: 30pt)

#slide[
    #set text(size: 35pt)
    #align(center)[🔈]
    #v(3em)
    #align(center)[*Introducing*]
    #v(1em)
    #align(center)[`trv`: Transform Video]

    #toolbox.pdfpc.speaker-note("
      Introducing trv.
      Transform Video.

      Transform slides and speaker notes into videos.
    ")
]

#slide[
    #v(3em)

    1. Slides -> Images

    2. Speaker notes -> Audio

    3. Audio + Images -> Video

    #toolbox.pdfpc.speaker-note("
      To do this, create a presentation with slides and speaker notes.

      Next, trv converts the slides into images and the speaker notes into audio.
      Finally, trv combines the audio and images into a video.
    ")
]


#slide[
    #v(2.5em)
    - Small file sizes

    - Automated video creation
    - Version control
    - Different languages/voices

    #toolbox.pdfpc.speaker-note("
      Benefits of using trv are that the files are small.
      The video that you are currently watching is less than 3 MB.
      This makes it easy to upload the video but also to self-host the video on platforms like GitHub-Pages.

      The second benefit is that the generation is fully automated.
      You just need to create a presentation and trv will take care of the rest.

      The third benefit is that you can store the source of the video in version control.
      This makes it easy to update the video or collaborate with others.

      The fourth benefit is that you can create videos in different languages and voices.
      This is useful if you are creating content for a global audience.

      You could even use it to tell jokes, because the text-to-speech engine is unlikely to laugh when telling a joke.
    ")
]

#slide[
    #set page(margin: 2em)
    #set text(size: 30pt)
    #align(left)[
      ```typ
      #import "@preview/polylux:0.4.0": *
      #set page(paper: "presentation-16-9")

      #slide[
        Hello

        #toolbox.pdfpc.speaker-note("
          This page contains Hello
        ")
      ]
      ```
    ]

    #toolbox.pdfpc.speaker-note("
      To create the presentation, we use Typst.
      Typst is a new typesetting system that is similar to LaTeX.

      Here for example is a simple Typst document with one slide.
      The slide contains the text Hello
      and a speaker note with the text This page contains Hello
    ")
]

#slide[
    #set page(margin: 3em)
    #set text(size: 24pt)
    ```bash
    $ trv \
      --input presentation.typ \
      --provider='openai-compatible(kokoros.transformrs.org)' \
      --model=tts-1 \
      --voice=af_sky \
      --audio-format=wav \
      --release
    ```

    #toolbox.pdfpc.speaker-note("
      To convert the presentation into a video, you can use the trv command line tool.

      The most important settings are related to the audio generation.

      Here we use the Kokoro text-to-speech model that is hosted at kokoros.transformrs.org.
      To run everything locally, you can also host this model yourself, see the README for more details.

      The input flag specifies the input file.
      This is the Typst presentation like the one we just saw.
      
      Next, the model, voice, and audio format settings are all related to the audio generation.

      Finally, the release flag ensures that a release.mp4 file is created that can be played on most devices, or uploaded to most platforms.
    ")
]


#slide[
    #v(2em)
    - Problem -> GitHub issue

    - Question -> GitHub issue
    - Feature Request -> GitHub issue

    #v(2em)
    #align(center)[
      #link("https://github.com/transformrs/trv")
    ]

    #toolbox.pdfpc.speaker-note("
      This is the end of the demo.
      Thanks for watching.

      If you have any questions or feedback, please open a GitHub issue.
    ")
]