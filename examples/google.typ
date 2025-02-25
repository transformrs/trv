#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

#slide[
    #toolbox.pdfpc.speaker-note("
    This video was created with the Google text-to-speech API.
    ")
    #set page(fill: rgb("#4285f7"))
    #v(6em)
    #set text(size: 35pt)
    #align(center)[#text(fill: white)[*Google text-to-speech*]]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    As an example, we can explain the following math problem.

    2 plus 2 equals 2x.
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    What is x in this equation?
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]

    #v(2em)
    #align(center)[What is x?]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    To solve it, we can move the 2x to the left.

    Or in other words, we put everything on the left side of the equation on the right side and everything on the right side of the equation on the left side.
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]

    #v(1em)
    #align(center)[2x = 2 + 2]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    Now we "move the 2 to the right".

    This can be done by dividing both sides of the equation by 2.

    Now we have x is 2 + 2 divided by 2.
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]

    #v(1em)
    #align(center)[2x = 2 + 2]

    #v(1em)
    #align(center)[x = $frac(2 + 2, 2)$]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    This gives us x is 4 divided by 2.
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]

    #v(1em)
    #align(center)[2x = 2 + 2]

    #v(1em)
    #align(center)[x = $frac(2 + 2, 2)$]

    #v(1em)
    #align(center)[x = $frac(4, 2)$]
]

#slide[
    #toolbox.pdfpc.speaker-note(
    ```md
    So, the answer is 2.
    ```
    )

    #v(2em)
    #align(center)[2 + 2 = 2x]

    #v(1em)
    #align(center)[2x = 2 + 2]

    #v(1em)
    #align(center)[x = $frac(2 + 2, 2)$]

    #v(1em)
    #align(center)[x = $frac(4, 2)$]

    #v(1em)
    #align(center)[x = 2]
]
