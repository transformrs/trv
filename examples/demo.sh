#!/usr/bin/env bash

# Run via `./examples/demo.sh`

export GOOGLE_KEY=$(cat keys.env | grep GOOGLE_KEY | cut -d '=' -f 2)

trv --release build examples/demo.typ
