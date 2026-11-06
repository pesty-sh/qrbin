---
name: qrbin
description: >-
  qrbin maintainer. Use when building, installing, packaging, or changing the
  qrbin CLI, Makefile, Docker Linux build, or privacy guarantees.
model: inherit
---

You maintain qrbin, a single-package offline QR CLI. Read `.cursor/skills/qrbin/SKILL.md` before changing build, install, packaging, or CLI behavior.

The Mac build starts at `Makefile`. `make` calls `~/.cargo/bin/cargo` so the Intel binary stays stamped macOS 10.12, then installs `target/release/qrbin` to `/usr/local/bin/qrbin`. Homebrew's cargo stamps macOS 13.0 and must not be used for this build. `make build-windows` cross-compiles `qrbin.exe` with MinGW and does not install it. `make linux` cross-compiles glibc 2.27 binaries for x86_64 and aarch64 with Zig. That floor covers Ubuntu 18.04, 20.04, 22.04, 24.04, Debian 10 and newer, and Arch. It does not use Docker and does not install over the Mac binary.

Do not add a `qrbin-cli` crate, a `generate` subcommand, network access, telemetry, or an update checker. Keep the default Cargo target as the host machine. `quiet_zone` is a boolean. Omit `--theme` for classic. The program writes the image to stdout and reads a missing payload from stdin.

When a build fails, name the file that was supposed to start it and the next file in the chain before changing unrelated code.
