---
name: qrbin
description: >-
  Build, install, and package the qrbin offline QR CLI. Use when working in
  the qrbin repo, or when the user mentions make, make deb, install, Ubuntu
  18.04, 22.04, 24.04, glibc, Docker, zigbuild, the Makefile, or the qrbin
  command line.
---

# qrbin

Single Rust package at the repo root. Binary and package name are `qrbin`. There is no `qrbin-cli` crate and no `generate` subcommand.

## Builds

Run from the repo root. `Makefile` starts both processes.

Mac install (`make`, default target `install`):

1. `build` runs `cargo build --release` with the `cargo` on PATH (Homebrew `/usr/local/bin/cargo` on the current Intel Mac).
2. Output is `target/release/qrbin`.
3. `install` copies it to `~/.local/bin/qrbin`.

`make build` stops before the copy. Re-running `make` replaces the installed binary. The program has no update checker and must not gain one.

Linux package (`make deb` or `make linux`):

1. `packaging/build-linux.sh` requires a running Docker daemon.
2. `packaging/Dockerfile` builds the `qrbin-linux-builder` image (Rust, Zig 0.16.0, `cargo-zigbuild`).
3. `packaging/build-inside.sh` runs `cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.27`.
4. `packaging/mkdeb.py` writes `dist/qrbin_<version>-1_amd64.deb`. The raw binary is `dist/qrbin-linux-amd64`.
5. Version comes from `Cargo.toml`. glibc 2.27 covers Ubuntu 18.04, 22.04, 24.04, and Debian with `libc6 (>= 2.27)`.

`rust-toolchain.toml` only lists extra rustup targets. It does not select the Cargo build target. Do not set a default `build.target`. The Mac build must stay the host machine.

## CLI

`qrbin [OPTIONS] [PAYLOAD]`. Image goes to stdout. Omitted payload is read from stdin.

| Flag | Values | Default |
|---|---|---|
| `-f`, `--format` | `png`, `svg` | `png` |
| `-s`, `--size` | minimum pixels | `512` |
| `-m`, `--margin` | non-zero keeps the border | `4` |
| `-e`, `--ecc` | `low`, `medium`, `quartile`, `high` | `medium` |

`qrcode` 0.14 `quiet_zone` takes a bool. Non-zero margin passes `true` and the library uses its fixed 4-module border. `0` passes `false`. `qrcode` features `image` and `svg` must stay enabled. SVG rendering uses `qrcode::render::svg::Color`, not raw strings.

## Release line

0.1.0 is the classic renderer only. Do not route the default command through `qr-code-styling`. Future `--type` defaults to `classic`. `themed` uses `qr-code-styling` 0.2.4 with the `png` feature only. Scope through 0.1.8 is in `CHANGELOG.md`. There is no 0.1.9. Linking the themed renderer increases binary size even for classic runs.

## Privacy limits

Do not add an HTTP client, telemetry, crash reporter, update check, remote config, or payload logging. Errors must not echo the payload. `missing_docs` warns for a missing crate-level `//!` comment in `src/main.rs`.

## Tests

`cargo test` compiles. `tests/cli_test.rs` is empty, so the suite reports zero tests until cases are added.
