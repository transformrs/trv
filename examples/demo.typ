#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set page(fill: black, margin: 2em)
#set text(fill: white)
#set text(size: 30pt)

#slide[
    #set text(size: 35pt)
    #align(center)[🔈]
    #v(4em)
    #align(center)[*Introducing Transform Video (trv)*]

    #toolbox.pdfpc.speaker-note("
      Introducing Transform Video (trv)

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

      First the slides are converted into images.
      Next, the speaker notes are converted into audio via a text-to-speech engine.
      Finally, the audio and images are combined into a video.
    ")
]

#slide[
    #set page(margin: 3em)
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