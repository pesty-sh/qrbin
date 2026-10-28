# Changelog

## 0.1.0 — 2026-10-28

Classic offline QR CLI. This tag does not include themed rendering.

- Encode text or bytes from an argument or stdin and write PNG or SVG to stdout.
- Flags: `--format`, `--size`, `--margin`, `--ecc`.
- No network client, telemetry, update check, payload logging, or default disk writes.
- `make` builds the host release binary and installs it to `~/.local/bin/qrbin`.
- `make deb` builds one glibc 2.27 amd64 `.deb` for Ubuntu 18.04, 22.04, and 24.04.

## Planned

Themed output stays off the classic path. `--type` will default to `classic`. `themed` will use `qr-code-styling` 0.2.4 with the `png` feature only. JPEG, WebP, PDF, and CMYK stay out of this line, so there is no 0.1.9.

| Version | Scope |
|---|---|
| 0.1.1 | Keep the classic renderer for the default command. Add the themed renderer behind `--type themed`. Settle quiet-zone behavior. Text payloads only on the themed path. |
| 0.1.2 | Named themes: `plain`, `rounded`, `dots`, `classy`. |
| 0.1.3 | Flat foreground and background colors. Errors must not echo the payload. |
| 0.1.4 | Two-stop linear and radial gradients on the dots. Background stays flat. |
| 0.1.5 | Optional circle shape. Document the scan cost and prefer high error correction. |
| 0.1.6 | Logo from a local file. No URL fetch. |
| 0.1.7 | Frame thickness, color, corner roundness, and an optional text label. No border images. |
| 0.1.8 | Local theme file via the crate’s `serde` feature. No remote config. |
