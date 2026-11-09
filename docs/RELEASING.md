# How a qrbin release is built

Nothing in [`.github/workflows/release.yml`](../.github/workflows/release.yml) runs until somebody pushes a tag that starts with `v`. Do not push that tag until you want a public GitHub Release. The current version is 0.1.9.

GitHub Actions runs those steps on GitHub's computers. This repository is public, so the standard runner minutes are not billed. A Mac job that dies while it is still queued is a full runner pool, not a bug in qrbin. Rerun that job. Do not change the code for it.

The local `make` commands stay. They are how this Mac builds a 10.12 Intel binary, an Apple silicon binary, a Windows exe, and the glibc 2.27 Linux packages. The workflows do not replace them.

## Cut a release

On `master`, after `version` in `Cargo.toml` is the version you want, and after the pull request CI is green:

```bash
git tag -a v0.1.9 -m "qrbin 0.1.9"
git push origin v0.1.9
```

`git push origin v0.1.9` sends that one tag. `git push --tags` would send every local tag.

The workflow only starts for tags that look like `v0.1.9` (the pattern is `v*`). It checks that the tag, without the `v`, matches `version` in `Cargo.toml`. A mismatch fails the job before it uploads anything.

## What happens, in order

### 1. Tag push

GitHub starts the Release workflow. Pull requests do not start it. Pull requests start [ci.yml](../.github/workflows/ci.yml): format, clippy, tests, the four musl targets, Windows, and cargo-deny. macOS is not in that list. Apple silicon runners queue, and the Intel runner was cancelled while queued on the sibling tools. Those builds live in their own workflows so a pull request cannot cancel them.

### 2. Matrix builds

Each row is one computer and one Rust target. They run at the same time.

| Rust target | Runner | Archive |
|---|---|---|
| `x86_64-apple-darwin` | optional Intel workflow | See section 4 |
| `aarch64-apple-darwin` | optional Apple silicon workflow | See section 4 |
| `x86_64-unknown-linux-musl` | Ubuntu x86_64 | `.tar.gz` plus `amd64` `.deb` |
| `aarch64-unknown-linux-musl` | Ubuntu arm64 | `.tar.gz` plus `arm64` `.deb` |
| `armv7-unknown-linux-musleabihf` | Ubuntu x86_64, cross-linked | `.tar.gz` plus `armhf` `.deb` |
| `i686-unknown-linux-musl` | Ubuntu x86_64, cross-linked | `.tar.gz` plus `i386` `.deb` |
| `x86_64-pc-windows-msvc` | Windows | `.zip` |

There is one macOS build per CPU family, not one per macOS version. The Intel binary asks for macOS 10.12. The Apple silicon binary asks for 11.0. Windows on ARM is not built.

Linux binaries in this workflow are static musl. They do not need the system's glibc, so the intended floor is Ubuntu 14.04 and Debian 8. CI itself runs on Ubuntu 24.04. armv7 and i686 are linked with `rust-lld` from the Rust toolchain (`packaging/rust-lld.sh`), not with a C cross compiler.

`make linux` is a different build. It runs on the Mac, links glibc 2.27, and writes `dist/qrbin-linux-amd64` and `dist/qrbin-linux-arm64`. Keep that path. It is the one the apt repository uses.

### 3. `.deb` packages

On each Linux leg, after the binary exists:

```bash
cargo deb --no-build --compress-type gzip --target <triple>
```

`--no-build` packs the binary just produced. `--compress-type gzip` writes `data.tar.gz`. Current cargo-deb defaults to xz, which Ubuntu 14.04 and Debian 8 dpkg may reject. The package metadata is `[package.metadata.deb]` in `Cargo.toml`. `depends` is empty because the musl binary does not need `libc6`.

| Rust target | `.deb` architecture |
|---|---|
| `x86_64-unknown-linux-musl` | amd64 |
| `aarch64-unknown-linux-musl` | arm64 |
| `armv7-unknown-linux-musleabihf` | armhf |
| `i686-unknown-linux-musl` | i386 |

`cargo-deb` is pinned to 3.6.0 and installed with `--no-default-features`, so the tool itself does not build liblzma. It is not linked into `qrbin`.

The filename looks like `qrbin_0.1.9-1_amd64.deb`. `make deb` on the Mac writes the same filename for the glibc build. They are not interchangeable. The GitHub Release carries the musl file. Apt publishes the glibc file. Do not put both in one apt pool.

### 4. Mac builds, optional

The main Release workflow does not build macOS.

| Workflow | When it runs | What it uploads on a tag |
|---|---|---|
| [release-apple-silicon.yml](../.github/workflows/release-apple-silicon.yml) | Actions → Run workflow, or a `v*` tag | `qrbin-vX.Y.Z-aarch64-apple-darwin.tar.gz` and a `.sha256` file |
| [release-macos-intel.yml](../.github/workflows/release-macos-intel.yml) | Actions → Run workflow, or a `v*` tag | `qrbin-vX.Y.Z-x86_64-apple-darwin.tar.gz` and a `.sha256` file |

Each has its own concurrency group and `cancel-in-progress: false`. `MACOSX_DEPLOYMENT_TARGET` is 10.12 on Intel and 11.0 on Apple silicon. `gh release upload --clobber` adds the files to the release the main workflow created, or creates that release if it is not there yet.

`make` on the Mac still produces the 10.12 Intel binary in `target/release/qrbin`. That file is not uploaded by these workflows.

### 5. Universal Mac binary, also optional

After both tarballs above are on the release, run [release-macos-universal.yml](../.github/workflows/release-macos-universal.yml) by hand. Type the tag, for example `v0.1.9`.

The job does not check out the repo, so it sets `GH_REPO` to `${{ github.repository }}`. It checks that the release exists, and if a tarball is missing it prints that tarball's name. Then it runs `lipo -create` and uploads `qrbin-universal-apple-darwin.tar.gz` plus its `.sha256`. It does not run on a tag by itself, because the two Mac jobs may still be queued.

Homebrew does not install this file. The formula compiles from source.

### 6. Checksums

One Ubuntu job downloads every archive and every `.deb` from the main matrix. It writes `SHA256SUMS` with `sha256sum`. That file lists the musl tarballs, the zip, and the four `.deb` files. It does not list itself, and it does not list the Mac archives, because those are uploaded by the other workflows. Each Mac archive has its own `.sha256` sidecar.

Check a download with:

```bash
sha256sum -c SHA256SUMS
```

### 7. GitHub Release

The same job creates the GitHub Release and attaches those files. The text starts from [RELEASE_NOTES_TEMPLATE.md](RELEASE_NOTES_TEMPLATE.md). The word `VERSION` is replaced by the Cargo version.

If any matrix leg fails, the release is not created.

The Windows zip is unsigned. SmartScreen warns the first time somebody runs `qrbin.exe`. That is the missing signature.

### 8. Homebrew and apt, by hand

The workflow stops at the GitHub Release. It cannot push to `pesty-sh/homebrew-tap` or `pesty-sh/apt`.

Homebrew, after the tag exists:

1. Download `https://github.com/pesty-sh/qrbin/archive/refs/tags/v0.1.9.tar.gz`.
2. `shasum -a 256` that file. Download it again and check the same hash a second time.
3. Put that hash in the tap formula. `url` is that tag tarball, not a binary.
4. The formula keeps `depends_on "rust" => :build` and `system "cargo", "install", *std_cargo_args`.
5. Run `brew style` and `brew audit --new` before the tap pull request.

For `v0.1.9` the checked hash is `afc109f18bc91139a26d1f881b1e590112ad53ad38e17b9d4dd2ab78c1a0e81e`. The copy in this repo is `packaging/homebrew/qrbin.rb`. No bottles.

apt: on the Mac that holds the signing key, pass the glibc `.deb` files from `make deb` to `publish.py` in `pesty-sh/apt`. `publish.py` refuses a package with an empty `Depends` line, so the musl GitHub `.deb` cannot replace the pool file. Cloud agents cannot sign. The private key must not leave that Mac.

crates.io: do not publish unless Stuart asks.

## What does not happen

- CI does not create the tag.
- crates.io is not updated.
- Binaries are not signed.
- There is no build per macOS version and no `.app`.
- JPEG, WebP, PDF, and CMYK are not in this release. There is no `--type` flag.
