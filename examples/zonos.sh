#!/usr/bin/env bash

# Run via `./examples/zonos.sh`

export DEEPINFRA_KEY=$(cat keys.env | grep DEEPINFRA_KEY | cut -d '=' -f 2)

trv build examples/zonos.typ
