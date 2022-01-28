#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

DOCKER_BUILDKIT=1 docker build -f "${SCRIPT_DIR}/Dockerfile" -t plabayo/news-web .
