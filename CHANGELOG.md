# Changelog

## Unreleased

- The Intel build calls rustup and keeps a macOS 10.12 stamp, so a 2014 Mac can run it. Homebrew's rustc stamps 13.0 and is not used.
- `make build-silicon` cross-compiles the Apple silicon binary and checks a macOS 11.0 stamp.
- `make build-windows` cross-compiles a 64-bit `qrbin.exe` for Windows 10 and 11. It does not install that file on the Mac.
- `make linux` cross-compiles glibc 2.27 builds for Intel/AMD and 64-bit ARM. That floor runs on Ubuntu 18.04, 20.04, 22.04, 24.04, Debian 10 through 13, and Arch. Docker is not used.
- Package metadata and `packaging/homebrew/qrbin.rb` point at GitHub `pesty-sh/qrbin`, homepage `https://pesty.sh`, and `pesty@pesty.sh`.
- `make` no longer installs. Fresh builds stay in `target/`. The Mac program on `PATH` is installed with Homebrew.

## 0.1.0 — 2026-10-28

Classic offline QR CLI. This tag does not include themed rendering.

- Encode text or bytes from an argument or stdin and write PNG or SVG to stdout.
- Flags: `--format`, `--size`, `--margin`, `--ecc`.
- No network client, telemetry, update check, payload logging, or default disk writes.
- `make` builds the host release binary and installs it to `/usr/local/bin/qrbin`.
- `make deb` builds one glibc 2.27 amd64 `.deb` for Ubuntu 18.04, 22.04, and 24.04.

## 0.1.8 — 2026-10-29

Themed rendering in the same binary. A command with no `--theme` and no `--theme-file` stays on the classic renderer.

- Themes: `plain`, `rounded`, `dots`, `classy`.
- Colors: `--fg`, `--bg`. Gradients: `--gradient linear|radial` with `--from` and `--to`. On a radial gradient, `--from` is the center and `--to` is the edge.
- `--shape circle`. `--logo` reads a local file and rejects a path that contains `://`.
- Border width `--frame` / `--border-width`, color `--frame-color` / `--border-color`, text `--label` / `--text`. A circle border is a ring.
- `--theme-file` reads local TOML. Command flags override the file.
- JPEG, WebP, PDF, and CMYK stay out, so there is no 0.1.9.

| Version | Scope |
|---|---|
| 0.1.1 | Keep the classic renderer for the default command. Add the themed renderer behind `--theme`. Settle quiet-zone behavior. Text payloads only on the themed path. |
| 0.1.2 | Named themes: `plain`, `rounded`, `dots`, `classy`. |
| 0.1.3 | Flat foreground and background colors. Errors must not echo the payload. |
| 0.1.4 | Two-stop linear and radial gradients on the dots. Background stays flat. |
| 0.1.5 | Optional circle shape. Document the scan cost and prefer high error correction. |
| 0.1.6 | Logo from a local file. No URL fetch. |
| 0.1.7 | Frame thickness, color, and an optional text label. No border images. |
| 0.1.8 | Local TOML theme file. Flags override the file. No remote config. |
