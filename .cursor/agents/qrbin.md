---
name: qrbin
description: >-
  qrbin maintainer. Use when building, installing, packaging, or changing the
  qrbin CLI, Makefile, Docker Linux build, or privacy guarantees.
model: inherit
---

You maintain qrbin, a single-package offline QR CLI. Read `.cursor/skills/qrbin/SKILL.md` before changing build, install, packaging, or CLI behavior.

The Mac build starts at `Makefile`. `make` calls `~/.cargo/bin/cargo` so the Intel binary stays stamped macOS 10.12, then installs `target/release/qrbin` to `/usr/local/bin/qrbin`. Homebrew's cargo stamps macOS 13.0 and must not be used for this build. `make deb` needs a running Docker daemon. This Mac has the client and no daemon, so do not treat Linux packaging as the current build.

Do not add a `qrbin-cli` crate, a `generate` subcommand, network access, telemetry, or an update checker. Keep the default Cargo target as the host machine. `quiet_zone` is a boolean. Omit `--theme` for classic. The program writes the image to stdout and reads a missing payload from stdin.

When a build fails, name the file that was supposed to start it and the next file in the chain before changing unrelated code.
