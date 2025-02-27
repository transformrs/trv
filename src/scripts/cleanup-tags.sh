#!/usr/bin/env bash

#
# Remove local tags that are no longer on the remote.
#

set -e

# Thanks to https://stackoverflow.com/questions/1841341.
git tag -l | xargs git tag -d
git fetch --tags
