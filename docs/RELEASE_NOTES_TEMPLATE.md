# qrbin VERSION

Offline QR generator. No telemetry, no update check, and no network access while it runs. The binaries are not signed.

## Minimum systems

These are the floors the binaries are built for. CI does not boot every old release.

| File | Runs on |
|---|---|
| `qrbin-vVERSION-x86_64-apple-darwin.tar.gz` | macOS 10.12 or newer, Intel. Added by the Intel workflow. |
| `qrbin-vVERSION-aarch64-apple-darwin.tar.gz` | macOS 11.0 or newer, Apple silicon. Added by the Apple silicon workflow. |
| `qrbin-universal-apple-darwin.tar.gz` | Both of the above, one file. Run the universal workflow by hand after both Mac tarballs exist. Not an `.app`. |
| `qrbin-vVERSION-x86_64-unknown-linux-musl.tar.gz` and `qrbin_VERSION-1_amd64.deb` | Linux amd64. Static musl. Ubuntu 14.04 / Debian 8 or newer. |
| `qrbin-vVERSION-aarch64-unknown-linux-musl.tar.gz` and `qrbin_VERSION-1_arm64.deb` | Linux arm64. Same floor. |
| `qrbin-vVERSION-armv7-unknown-linux-musleabihf.tar.gz` and `qrbin_VERSION-1_armhf.deb` | Linux armhf (32-bit ARM). Same floor. |
| `qrbin-vVERSION-i686-unknown-linux-musl.tar.gz` and `qrbin_VERSION-1_i386.deb` | Linux i386. Same floor. |
| `qrbin-vVERSION-x86_64-pc-windows-msvc.zip` | Windows 10 or Windows Server 2016, 64-bit. Unsigned. SmartScreen warns on the first run. |

The `.deb` files in this release are gzip-compressed static musl builds. They do not depend on glibc. The apt repository is a separate glibc 2.27 package built with `make deb`. Do not mix the two files that share a name.

Mac archives are not listed in `SHA256SUMS`. Each one has a `.sha256` file next to it. `SHA256SUMS` covers the musl archives, the `.deb` files, and the Windows zip.

## Checksums

```bash
sha256sum -c SHA256SUMS
```

On Windows, `Get-FileHash file -Algorithm SHA256` and compare the digest to the matching line.

Windows on ARM is not in this release.

JPEG, WebP, PDF, and CMYK are not output formats. There is no `--type` flag.
