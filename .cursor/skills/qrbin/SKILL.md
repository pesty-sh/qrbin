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

1. `build` runs `~/.cargo/bin/cargo` and pins `RUSTC` to `rustup which rustc`. The proxy loads rustup's `libLLVM` so release stripping works. The pin stops a cross build from selecting `/usr/local/bin/cargo`. That Homebrew compiler stamps macOS 13.0 and has no Apple silicon standard library.
2. `MACOSX_DEPLOYMENT_TARGET` is `10.12`. `packaging/check-macos-minos.sh` fails the build when `target/release/qrbin` is stamped newer than 10.12.
3. Output is `target/release/qrbin`.
4. `install` copies it to `/usr/local/bin/qrbin`. Do not also install a copy under `~/.local/bin`. On this Intel Mac that path is also Homebrew's prefix, so `brew install pesty-sh/tap/qrbin` would replace the same file.
5. The Homebrew formula is `packaging/homebrew/qrbin.rb`. Homepage `https://pesty.sh`, source `https://github.com/pesty-sh/qrbin`, tap `pesty-sh/homebrew-tap`, contact `pesty@pesty.sh`. The formula uses `cargo install --locked`. It must not run `make install`. Homebrew's own Rust stamps macOS 13.0, so a bottle is not the 2014 build.

`make build` stops before the copy. `make build-silicon` cross-compiles `aarch64-apple-darwin` and checks a macOS 11.0 stamp. It does not install. `make build-windows` cross-compiles `x86_64-pc-windows-gnu` with the MinGW linker `x86_64-w64-mingw32-gcc` (`brew install mingw-w64`) and checks that `target/x86_64-pc-windows-gnu/release/qrbin.exe` is a PE32+ exe. It does not install. Do not set Cargo's default target to Windows. `make doctor` prints which cargo is used and whether the Windows linker is present. Re-running `make` replaces the installed Mac binary. The program has no update checker and must not gain one.

Linux package (`make deb` or `make linux`) runs `packaging/build-linux-host.sh` on this Mac. `LINUX_GLIBC` is `2.27`, the Ubuntu 18.04 version. The same binary runs on Ubuntu 20.04 (2.31), 22.04 (2.35), 24.04 (2.39), Debian 10 (2.28) through Debian 13 (2.41), and Arch, because those ship a newer glibc. Intel and AMD are one `x86_64-unknown-linux-gnu.2.27` build. 64-bit ARM is `aarch64-unknown-linux-gnu.2.27`. Output is `dist/qrbin-linux-amd64`, `dist/qrbin-linux-arm64`, and a `.deb` for each. The script downloads Zig 0.16.0 from ziglang.org into `~/.cache/qrbin` and uses `cargo-zigbuild`. Do not `brew install zig`. Do not install the Linux binaries over `/usr/local/bin/qrbin`. `packaging/build-linux.sh` still calls Docker and is not what `make` runs.

`rust-toolchain.toml` only lists extra rustup targets. It does not select the Cargo build target. Do not set a default `build.target`. The Mac build must stay the host machine. Homebrew's cargo ignores `rust-toolchain.toml`.

## CLI

`qrbin [OPTIONS] [PAYLOAD]`. Image goes to stdout. Omitted payload is read from stdin.

| Flag | Values | Default |
|---|---|---|
| `-f`, `--format` | `png`, `svg` | `png` |
| `-s`, `--size` | minimum pixels | `512` |
| `-m`, `--margin` | non-zero keeps the border | `4` |
| `-e`, `--ecc` | `low`, `medium`, `quartile`, `high` | `medium` |
| `--theme` | `plain`, `rounded`, `dots`, `classy` | classic when omitted |
| `--theme-file` | local TOML | none |
| `--fg`, `--bg` | hex | black, white |
| `--gradient`, `--from`, `--to` | `linear` or `radial`, both stops | none |
| `--shape` | `square`, `circle` | `square` |
| `--logo` | local file | none |
| `--frame`, `--border-width` | border width in pixels | none |
| `--frame-color`, `--border-color` | hex | black |
| `--label`, `--text` | border text | none |

`qrcode` 0.14 `quiet_zone` takes a bool. Non-zero margin passes `true` and the library uses its fixed 4-module border. `0` passes `false`. `qrcode` features `image` and `svg` must stay enabled. SVG rendering uses `qrcode::render::svg::Color`, not raw strings.

Release 0.1.8. Do not route a command without `--theme` or `--theme-file` through `qr-code-styling`. `--theme` selects `qr-code-styling` 0.2.4 with the `png` feature only. Do not add `--type`. There is no 0.1.9. Linking the themed renderer increases binary size even for classic runs.

## Theme arguments

Omit `--theme` and `--theme-file` for the `qrcode` renderer. Otherwise use `qr-code-styling` 0.2.4 with the `png` feature only. Themes are `plain` (square modules and finders), `rounded` (rounded modules, extra-rounded finder squares, dot centers), `dots` (circular modules and finders), and `classy` (classy-rounded modules, extra-rounded finder squares, dot centers).

Style flags require `--theme` or a `theme` key in `--theme-file`. Command flags win over the file. Theme-file keys are `theme`, `fg`, `bg`, `gradient`, `from`, `to`, `shape`, `logo`, `frame`, `frame_color`, and `label`. Unknown keys are rejected. `format`, `size`, `margin`, and `ecc` stay on the command line.

`--fg` and `--bg` are hex, with or without `#`, at 3, 6, or 8 digits. Defaults are `#000000` and `#ffffff`. Finders inherit `--fg`. `--gradient` is `linear` or `radial` and needs both `--from` and `--to`. `--from` is the linear start and the radial center. `--to` is the linear end and the radial edge. The background stays flat.

`--shape` is `square` or `circle`. A circle adds rim modules. `--logo` reads a local file. A path containing `://` is rejected and is not written into the SVG. Use `--ecc high` with a logo or a circle. `--ecc` is always passed through. The styling crate's own default is quartile.

`--frame` and `--border-width` are the border width in pixels. `--frame-color` and `--border-color` default to black. `--label` and `--text` are the border text. Color or text without a width is an error. On a square the text sits on the bottom edge. On a circle the border is a ring, the mark is inset by the width, and the text follows the lower arc in the background color, sized from the width. Do not add border images.

Classic `--size` is a minimum and the PNG is grayscale. Themed `--size` is exact and the PNG is RGBA. A non-zero themed margin is `size / 16` pixels. `0` omits that pad. Errors must not echo the payload. No URL fetch. No remote config. The logo file is `assets/logo.png`. The SVG source is `assets/logo.svg`. Local binary is `./target/release/qrbin`. Installed binary is `/usr/local/bin/qrbin`.

## Privacy limits

Do not add an HTTP client, telemetry, crash reporter, update check, remote config, or payload logging. Errors must not echo the payload. `missing_docs` warns for a missing crate-level `//!` comment in `src/main.rs`.

## Tests

`cargo test` runs `tests/cli_test.rs`. `library_examples` writes the curated PNGs in the README appendix to `library/`. Logo examples use `assets/logo.png`. `binary_does_not_phone_home` reads the built binary, rejects tracker hosts and network libraries, and runs a classic and a themed command with the network denied.
