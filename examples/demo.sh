#!/usr/bin/env bash

# Run via `./examples/demo.sh`

export GOOGLE_KEY=$(cat keys.env | grep GOOGLE_KEY | cut -d '=' -f 2)

trv --input=examples/demo.typ \
    --provider=google \
    --voice=en-US-Chirp3-HD-Orus \
    --language-code=en-US \
    --release