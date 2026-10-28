# qrbin

**A minimal, offline-first QR generator.**

`qrbin` 0.1.0 reads a payload and writes a QR image to stdout. It has no telemetry, analytics, update check, account system, or outbound network connection.

```text
Read payload → generate QR → write result to stdout → exit
```

The running program does not phone home. `cargo build` and `make deb` may use the network on the build machine to fetch crates or a Docker image.

## Limits

`qrbin` does not log the payload, and error text does not include it. By default it writes only to stdout. `--output` does not exist. A file redirect such as `> qr.png` is the shell, not qrbin.

It cannot protect a payload from the operating system, shell history, a compromised host, swap, or a core dump. Command-line arguments can appear in process listings. Prefer stdin for anything sensitive.

## Install and build

Run these from the repository root. [`Makefile`](Makefile) starts both builds. [`Cargo.toml`](Cargo.toml) is the only package. There is no `qrbin-cli` crate. [`rust-toolchain.toml`](rust-toolchain.toml) tells rustup which extra targets to install. It does not change the default build target.

The program does not update itself. Run `make` again, or install a newer `.deb`.

### This Mac

```bash
make
qrbin --version
```

1. The `install` target depends on `build`.
2. `build` runs `cargo build --release`.
3. The binary is `target/release/qrbin`.
4. `install` copies it to `~/.local/bin/qrbin`.

`make build` stops after step 3. On this machine, `make` uses the `cargo` first on `PATH`.

### Ubuntu 18.04, 22.04, and 24.04

Start Docker, then:

```bash
make deb
sudo dpkg -i dist/qrbin_0.1.0-1_amd64.deb
```

`make linux` is the same target. [`packaging/build-linux.sh`](packaging/build-linux.sh) builds the image from [`packaging/Dockerfile`](packaging/Dockerfile), runs [`packaging/build-inside.sh`](packaging/build-inside.sh), and [`packaging/mkdeb.py`](packaging/mkdeb.py) writes the package. The binary is also at `dist/qrbin-linux-amd64`.

It is linked against glibc 2.27, so the same amd64 package installs on Ubuntu 18.04, 22.04, 24.04, and Debian with `libc6 (>= 2.27)`. The filename follows the version in `Cargo.toml`.

## Usage

```text
qrbin [OPTIONS] [PAYLOAD]
```

```bash
qrbin "https://bryter.digital" --format png --size 256 > qr.png
echo "https://bryter.digital" | qrbin --format png > qr.png
qrbin "https://bryter.digital" --format svg > qr.svg
qrbin --format png < invoice.txt > invoice-qr.png
qrbin "https://bryter.digital" --ecc high --margin 0 > qr.png
```

| Option | Meaning | Default |
|---|---|---|
| `-f`, `--format` | `png` or `svg` | `png` |
| `-s`, `--size` | Minimum width and height in pixels | `512` |
| `-m`, `--margin` | Non-zero keeps the standard 4-module quiet zone; `0` omits it | `4` |
| `-e`, `--ecc` | `low`, `medium`, `quartile`, or `high` | `medium` |

| ECC | Meaning |
|---|---|
| `low` | Lowest correction, highest capacity |
| `medium` | Default |
| `quartile` | Higher resilience |
| `high` | Highest resilience |

## Release 0.1.0

This release is the classic renderer: the `qrcode` crate plus a PNG or SVG writer. The command above stays on that path. See [CHANGELOG.md](CHANGELOG.md).

## Theming scope

Later releases add style without changing the classic command. `--type` will default to `classic`. `--type themed` will use `qr-code-styling` 0.2.4 with only its `png` feature. SVG rendering comes with that crate. JPEG, WebP, PDF, and CMYK stay disabled.

A classic run will still produce the same grayscale PNG. The executable grows once the themed renderer is linked, because the flag can reach `resvg` even when the user asks for classic. A separate Cargo feature is how a small classic-only binary stays available.

The crate does not read the network or send telemetry. Logos must be bytes already in memory. Border images are excluded because that API copies a string into an SVG link. Themed payloads are text. Its error-correction default is Q, so qrbin must pass `--ecc` through. Its margin is in pixels, so 0.1.1 has to define how `--margin` behaves before any theme ships.

Styled dots, circles, and logos cover data modules. Use `--ecc high` for those.

| Version | Scope |
|---|---|
| 0.1.1 | Classic path unchanged. Themed renderer behind `--type themed`. Quiet zone settled. |
| 0.1.2 | Themes: `plain`, `rounded`, `dots`, `classy`. |
| 0.1.3 | Flat foreground and background colors. |
| 0.1.4 | Two-stop linear and radial gradients on the dots. Background stays flat. |
| 0.1.5 | Optional circle shape. |
| 0.1.6 | Logo from a local file. No URL fetch. |
| 0.1.7 | Frame thickness, color, roundness, and an optional text label. |
| 0.1.8 | Local theme file. No remote config. |

There is no 0.1.9. Print formats belong in a later major line if they are needed.

## Development

Requirements: Rust stable, Cargo, and Git. `make deb` also needs a running Docker daemon.

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

`tests/cli_test.rs` is empty, so `cargo test` currently reports no tests.

## Licence

MIT
