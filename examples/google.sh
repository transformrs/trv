#!/usr/bin/env bash

# Run via `./examples/google.sh`

export GOOGLE_KEY=$(cat keys.env | grep GOOGLE_KEY | cut -d '=' -f 2)

cp examples/math.typ _out/math.typ

trv --input=examples/google.typ \
    --provider=google \
    --voice=en-US-Chirp-HD-D \
    --language-code=en-US \
    --release
