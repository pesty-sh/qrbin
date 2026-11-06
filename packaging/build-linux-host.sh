#!/bin/sh
# Cross-compile glibc Linux binaries on this Mac. Does not use Docker
# and does not install over the Mac binary.
set -eu

cd "$(dirname "$0")/.."

glibc="${1:-2.27}"
zig_version="0.16.0"
rustup="${HOME}/.cargo/bin/rustup"
cargo="${HOME}/.cargo/bin/cargo"

if [ ! -x "$rustup" ] || [ ! -x "$cargo" ]; then
    echo "rustup cargo is missing." >&2
    exit 1
fi

export RUSTC="$("$rustup" which rustc)"
unset MACOSX_DEPLOYMENT_TARGET
unset CARGO_TARGET_DIR

mach="$(uname -m)"
case "$mach" in
    arm64) zig_arch="aarch64" ;;
    *) zig_arch="$mach" ;;
esac

zig_root="${HOME}/.cache/qrbin/zig-${zig_arch}-macos-${zig_version}"
zig="${zig_root}/zig"
if [ ! -x "$zig" ]; then
    mkdir -p "${HOME}/.cache/qrbin"
    url="https://ziglang.org/download/${zig_version}/zig-${zig_arch}-macos-${zig_version}.tar.xz"
    echo "downloading ${url}"
    curl -fsSL "$url" | tar -xJ -C "${HOME}/.cache/qrbin"
fi
export PATH="${zig_root}:${HOME}/.cargo/bin:${PATH}"

if [ ! -x "${HOME}/.cargo/bin/cargo-zigbuild" ]; then
    "$cargo" install --locked cargo-zigbuild
fi

check_glibc() {
    binary="$1"
    label="$2"
    file "$binary" | grep -q "ELF 64-bit" || {
        echo "${binary} is not a 64-bit ELF." >&2
        exit 1
    }
    file "$binary" | grep -q "$label" || {
        echo "${binary} is not ${label}." >&2
        file "$binary" >&2
        exit 1
    }
    newest="$(strings "$binary" | sed -n 's/^GLIBC_//p' | sort -t. -k1,1n -k2,2n -k3,3n | tail -n 1)"
    top="$(printf '%s\n%s\n' "$glibc" "$newest" | sort -t. -k1,1n -k2,2n -k3,3n | tail -n 1)"
    if [ "$top" != "$glibc" ]; then
        echo "${binary} needs GLIBC_${newest}, newer than ${glibc}." >&2
        exit 1
    fi
    echo "${binary}: ELF ${label}, glibc ${newest}"
}

build_one() {
    rust_target="$1"
    deb_arch="$2"
    dist_name="$3"
    file_label="$4"

    "$rustup" target add "$rust_target"
    "$cargo" zigbuild --release --target "${rust_target}.${glibc}"

    binary="target/${rust_target}/release/qrbin"
    check_glibc "$binary" "$file_label"

    mkdir -p dist
    cp "$binary" "dist/${dist_name}"
    chmod 0755 "dist/${dist_name}"

    version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
    python3 packaging/mkdeb.py \
        "dist/${dist_name}" \
        "$version" \
        "$deb_arch" \
        "dist/qrbin_${version}-1_${deb_arch}.deb"
    echo "wrote dist/qrbin_${version}-1_${deb_arch}.deb"
}

build_one x86_64-unknown-linux-gnu amd64 qrbin-linux-amd64 "x86-64"
build_one aarch64-unknown-linux-gnu arm64 qrbin-linux-arm64 "aarch64"
