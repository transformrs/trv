#!/usr/bin/env bash

set -euo pipefail

export DEEPINFRA_KEY="$(cat test.env | grep DEEPINFRA_KEY | cut -d= -f2)"

echo $DEEPINFRA_KEY

PROMPT="
transformrs is a Rust interface to multiple AI providers.
It supports chat, text-to-image, and text-to-speech.

For 
"

echo $PROMPT | trf tts --model=hexgrad/Kokoro-82M \
    --voice=am_eric \
    --output=tmp.mp3
