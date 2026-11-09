---
name: release
description: >-
  Cut a qrbin release. Use when the version in Cargo.toml changes, when
  tagging v*, when editing .github/workflows or docs/RELEASING.md, or when
  preparing the Homebrew or apt handoff.
---

# qrbin release

Current version is **0.1.9**. `Cargo.toml` `version` must match the tag without the leading `v`. Tag `v0.1.9` matches `0.1.9`.

This skill sets up the release. It does not publish one. Do not push a tag, publish to crates.io, or run apt `publish.py` unless Stuart asks. A `v*` tag starts the public GitHub Release.

Read `.cursor/skills/qrbin/SKILL.md` before changing the Makefile, the glibc Linux build, or the CLI. JPEG, WebP, PDF, and CMYK stay out. There is no `--type` flag.

## What is already true

- Default branch is `master`.
- `make` still builds the Intel Mac binary with rustup and a macOS 10.12 stamp. It does not install. `make build-silicon`, `make build-windows`, and `make linux` stay as they are. `make linux` is the glibc 2.27 build (Ubuntu 18.04 and newer). Docker is not used.
- GitHub Actions is a second path. On a `v*` tag it builds static musl Linux (x86_64, aarch64, armv7, i686), gzip `.deb` files, and a Windows zip. macOS is not on the pull-request path.
- Homebrew stays a source build. No bottles. The formula in this repo is `packaging/homebrew/qrbin.rb`. The tap is `pesty-sh/homebrew-tap`. Fill `sha256` only from a real download of the tag tarball, then check that hash a second time.
- Apt packages are signed on Stuart's Mac by `publish.py` in `pesty-sh/apt`. The GPG private key stays there. Hand him the glibc `.deb` files from `make deb`. Do not upload the musl GitHub `.deb` into that pool under the same filename.

## Files

| File | Job |
|---|---|
| `.github/workflows/ci.yml` | Pull requests and pushes to `master`. fmt, clippy, tests, four musl targets, Windows msvc, cargo-deny. `cancel-in-progress: true`. |
| `.github/workflows/release.yml` | Tag `v*` only. Musl tarballs, gzip `.deb`, Windows zip, `SHA256SUMS`, GitHub Release. |
| `.github/workflows/release-macos-intel.yml` | `workflow_dispatch` and tag `v*`. `MACOSX_DEPLOYMENT_TARGET=10.12`. Own concurrency group, `cancel-in-progress: false`. `gh release upload --clobber`. |
| `.github/workflows/release-apple-silicon.yml` | Same, with `MACOSX_DEPLOYMENT_TARGET=11.0`. |
| `.github/workflows/release-macos-universal.yml` | `workflow_dispatch` with a `tag` input. No checkout. `GH_REPO: ${{ github.repository }}`. `lipo -create`. Name a missing release or tarball in the failure. |
| `docs/RELEASING.md` | The map of those files. |
| `docs/RELEASE_NOTES_TEMPLATE.md` | Body of the GitHub Release. The word `VERSION` is replaced with the Cargo version. |
| `deny.toml` | `cargo deny check`. |
| `packaging/homebrew/qrbin.rb` | Source formula. `url` stays commented until the tag exists and the checksum is real. |

Lines that change when this file is copied to another pesty-sh tool are marked `# CHANGE ME:`.

## Cutting 0.1.9

1. Version in `Cargo.toml`, `CHANGELOG.md`, and the README match.
2. Open a pull request. Run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` before the commit. Author and committer are `stuarteske <sdeske@googlemail.com>`.
3. Merge only when CI is fully green. A Mac job that fails because the runner queue was full is not a code bug. Rerun it. Do not change code for that.
4. On `master`, after the merge: `git tag -a v0.1.9 -m "qrbin 0.1.9"` then `git push origin v0.1.9`. Do not use `git push --tags`.
5. Wait until `release.yml` has succeeded. Run the Intel and Apple silicon workflows if the tag did not finish them. Then run the universal workflow and type `v0.1.9`.
6. Check the assets and `SHA256SUMS`. Mac sidecars are `*.sha256` files. They are not inside the main `SHA256SUMS`, because those Mac jobs upload later.
7. Homebrew and apt are separate commits in their own repos. This repo cannot push them.

## Do not

- Force-push or rewrite a pushed tag.
- Clear the `deny.toml` ignores for `rustybuzz` and `ttf-parser` by dropping `qr-code-styling`. Those two advisories have no fixed release. They come in through the themed renderer.
- Add an update checker, an HTTP client, or a crates.io publish.
- Point the Homebrew formula at a prebuilt binary.
- Let `make` install over `/usr/local/bin/qrbin`.
- Treat a cancelled macOS runner as a failed test of the Rust code.
