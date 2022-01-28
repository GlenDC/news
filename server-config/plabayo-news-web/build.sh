#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

docker build -f "${SCRIPT_DIR}/Dockerfile" -t plabayo/news-web .
