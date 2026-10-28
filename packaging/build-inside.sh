#!/bin/sh
# Build an amd64 binary inside the Linux builder image and pack a .deb.
# glibc 2.27 is Ubuntu 18.04's version, so the same package runs on 18.04,
# 22.04, and 24.04.
set -eu

cd /src
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
export CARGO_TARGET_DIR=/tmp/qrbin-target

cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.27

binary="${CARGO_TARGET_DIR}/x86_64-unknown-linux-gnu/release/qrbin"
mkdir -p /src/dist
cp "$binary" /src/dist/qrbin-linux-amd64
chmod 0755 /src/dist/qrbin-linux-amd64
python3 packaging/mkdeb.py \
    /src/dist/qrbin-linux-amd64 \
    "$version" \
    amd64 \
    "/src/dist/qrbin_${version}-1_amd64.deb"

echo "wrote /src/dist/qrbin_${version}-1_amd64.deb"
