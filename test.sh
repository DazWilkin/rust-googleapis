#!/bin/bash

PROJECT="crate-transparency"
TAG=$(git rev-parse HEAD)


# Uses ./src to avoid uploading (much) context
# No context is used by the Dockerfile
# Dockerfile installs protoc and git clones the repo
docker build \
--tag=gcr.io/${PROJECT}/rust-googleapis:${TAG} \
--file=./Dockerfile.test \
./src