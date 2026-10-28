# Build for this Mac, install that binary, or package Linux .deb files.
# `make` builds the release binary for the current machine and installs it
# to ~/.local/bin, which is ahead of /usr/local/bin on PATH. Run it again
# after a change to update the installed command.

PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin

.PHONY: build install deb linux

.DEFAULT_GOAL := install

build:
	cargo build --release

install: build
	mkdir -p "$(BINDIR)"
	install -m 0755 target/release/qrbin "$(BINDIR)/qrbin"
	@"$(BINDIR)/qrbin" --version
	@echo "updated $(BINDIR)/qrbin"

# One amd64 package, linked for glibc 2.27. It installs on Ubuntu 18.04,
# 22.04, and 24.04, and on Debian releases with glibc 2.27 or newer.
deb linux:
	./packaging/build-linux.sh
