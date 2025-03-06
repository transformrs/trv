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
    #v(8em)
    #set text(size: 35pt)
    #align(center)[*Text to video*]

    #toolbox.pdfpc.speaker-note("
      What if you could easily generate videos from text?
      foo
    ")
]

#slide[
    #set page(fill: black, margin: 3em)
    #set text(fill: white)
    #set text(size: 45pt)
    #align(left)[
      *Step 1:* Generate videos

      *Step 2:* ...?
      
      *Step 3:* Profit!
    ]

    #toolbox.pdfpc.speaker-note("
      That would be pretty cool.
      Here is a plan to make it happen.
    ")
]