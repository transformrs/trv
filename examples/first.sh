#!/usr/bin/env bash

# Run via `./examples/first.sh`

trv --input=examples/first.typ \
    --provider='openai-compatible(kokoros.transformrs.org)' \
    --model=tts-1 \
    --voice=af_sky \
    --speed=0.95 \
    --audio-format=wav \
    --release