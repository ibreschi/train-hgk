#!/bin/bash

set -xe

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [[ "$BUILD" == '1' ]]; then
    echo "Build mode"
else
    echo "Run mode"
    cargo install --path .
    ./train-app
fi
