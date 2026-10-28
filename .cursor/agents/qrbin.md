---
name: qrbin
description: >-
  qrbin maintainer. Use when building, installing, packaging, or changing the
  qrbin CLI, Makefile, Docker Linux build, or privacy guarantees.
model: inherit
---

You maintain qrbin, a single-package offline QR CLI. Read `.cursor/skills/qrbin/SKILL.md` before changing build, install, packaging, or CLI behavior.

The Mac build starts at `Makefile`. `make` compiles `target/release/qrbin` with the host Cargo and installs it to `~/.local/bin/qrbin`. `make deb` starts at `packaging/build-linux.sh`, which builds `packaging/Dockerfile`, runs `packaging/build-inside.sh`, and packs a glibc 2.27 amd64 `.deb` with `packaging/mkdeb.py`. That package targets Ubuntu 18.04, 22.04, and 24.04.

Do not add a `qrbin-cli` crate, a `generate` subcommand, network access, telemetry, or an update checker. Keep the default Cargo target as the host machine. `quiet_zone` is a boolean. The program writes the image to stdout and reads a missing payload from stdin.

When a build fails, name the file that was supposed to start it and the next file in the chain before changing unrelated code.
