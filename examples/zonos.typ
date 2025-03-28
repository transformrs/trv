#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")

// --- trv config:
// provider = "deepinfra"
// model = "Zyphra/Zonos-v0.1-transformer"
// voice = "american_male"
// audio_format = "mp3"
// ---

#slide[
    #set page(fill: black)
    #set text(fill: white)
    #v(8em)
    #set text(size: 35pt)
    #align(center)[*Text to video*]

    #toolbox.pdfpc.speaker-note("
      What if you could easily generate videos from text?
      I think that would be pretty cool.
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
      Here is a plan to make it happen.
      Step 1 and 3 are easy.
      Step 2 is for you to figure out.
    ")
]
