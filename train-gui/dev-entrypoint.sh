#!/bin/bash

set -xe

export DEBUG=1
alias serve="trunk serve"

export HOME=/tmp/home
mkdir -p $HOME

exec "$@"
