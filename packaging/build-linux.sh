#!/bin/sh
# Build the Ubuntu/Debian package. Requires Docker.
set -eu

cd "$(dirname "$0")/.."

if ! command -v docker >/dev/null 2>&1; then
    echo "docker is required to build the Linux package" >&2
    exit 1
fi

docker build -t qrbin-linux-builder -f packaging/Dockerfile packaging
mkdir -p dist
docker run --rm \
    --user "$(id -u):$(id -g)" \
    -e HOME=/tmp \
    -v "$PWD":/src \
    -w /src \
    qrbin-linux-builder \
    sh packaging/build-inside.sh
