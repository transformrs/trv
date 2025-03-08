#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")
#set text(size: 25pt)

// --- trv config:
// provider = "google"
// voice = "en-US-Chirp-HD-D"
// language_code = "en-US"
// ---

#slide[
    first

    #toolbox.pdfpc.speaker-note("
      OpenAI whisper is a tool that can be used to run speech recognition.

      It's a great tool for generating SRT files.
      
      In this video, I'll quickly show you how to use it.
    ")
]

#slide[
    second

    #toolbox.pdfpc.speaker-note("
      To install OpenAI whisper, there are multiple options.

      OpenAI advises to use pip install, but on MacOS it's probably easier to use Homebrew.

      Note that this installation might take a while.

      In case of problems during installation, see the openai whisper repository on GitHub.
    ")
]

#slide[
    third

    #toolbox.pdfpc.speaker-note("
      Usage should be pretty straightforward.

      Specify the audio file that you want to convert to SRT, and specify the model that you want to use.

      On the first run, the model will be downloaded automatically.

      Here I'm using the turbo model since that is usually the best option.

      If everything goes well, this command will generate a file called audio.srt.
    ")
]

#slide[
    fourth

    #toolbox.pdfpc.speaker-note("
      The turbo model requires 6 GB of video memory.

      If you want to use less video memory, then use the tiny, base, small, or medium model.

      Whisper offers two model variants: English-specific models and multilingual models.

      If you need only english, then use an english-only model such as small.en.
    ")
]

#slide[
    fifth

    #toolbox.pdfpc.speaker-note("
      Overall, whisper is a great tool for generating SRT files.

      But it's not perfect.

      It's usually a good idea to manually review the generated SRT file.
      Think of whisper as a starting point.
      It will have done 95% of the work for you, it's up to you to verify correctness.
    ")
]
