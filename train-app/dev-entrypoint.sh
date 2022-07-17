#!/bin/bash

set -xe

export DEBUG=1
alias build="cargo install --path ."

export HOME=/tmp/home
mkdir -p $HOME

exec "$@"
