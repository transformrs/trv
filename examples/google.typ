#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// --- trv config:
// provider = "google"
// voice = "en-US-Chirp-HD-D"
// language_code = "en-US"
// ---

#slide[
    #toolbox.pdfpc.speaker-note("
    This video was created with the Google text-to-speech API.
    ")
    #set page(fill: rgb("#4285f7"))
    #v(6em)
    #set text(size: 35pt)
    #align(center)[#text(fill: white)[*Google text-to-speech*]]
]

#include "math.typ"