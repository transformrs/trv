#!/usr/bin/env bash

set -euo pipefail

export DEEPINFRA_KEY="$(cat test.env | grep DEEPINFRA_KEY | cut -d= -f2)"

PROMPT="
What if you could show code not just in an example, but in a video?

Videos are useful in transfering lots of information, especially to provide context to code examples.
"

echo $PROMPT | trf tts --model=hexgrad/Kokoro-82M \
    --voice=am_eric \
    --output=tmp.mp3
