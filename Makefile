# Build for this Mac and install that binary.
# Intel Macs from 2014 can run macOS 10.12. rustup's standard library is
# stamped 10.12. The cargo first on PATH is Homebrew's rustc, and that
# standard library is stamped 13.0, so this file calls rustup by path.
# Do not set Cargo's default target. This Mac stays the host build.

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
RUSTUP := $(HOME)/.cargo/bin/rustup
# The proxy at ~/.cargo/bin/cargo loads rustup's libLLVM so release
# stripping works. RUSTC is pinned to the same toolchain so a cross
# build cannot pick up Homebrew's rustc from PATH.
RUSTC := $(shell $(RUSTUP) which rustc)
RUSTUP_CARGO := $(HOME)/.cargo/bin/cargo
export RUSTC
export MACOSX_DEPLOYMENT_TARGET := 10.12

.PHONY: build install doctor build-silicon deb linux

.DEFAULT_GOAL := install

build:
	@test -x "$(RUSTC)" || { \
		echo "rustup rustc is missing. Homebrew's rustc stamps macOS 13.0."; \
		exit 1; \
	}
	@test -x "$(RUSTUP_CARGO)" || { \
		echo "rustup cargo is missing."; \
		exit 1; \
	}
	"$(RUSTUP_CARGO)" build --release
	./packaging/check-macos-minos.sh target/release/qrbin 10.12

install: build
	mkdir -p "$(BINDIR)"
	install -m 0755 target/release/qrbin "$(BINDIR)/qrbin"
	@"$(BINDIR)/qrbin" --version
	@echo "updated $(BINDIR)/qrbin"

# Apple silicon. Does not install, because this Intel Mac cannot run it.
# The standard library for this target is stamped macOS 11.0.
build-silicon:
	@test -x "$(RUSTC)" || { echo "rustup rustc is missing."; exit 1; }
	MACOSX_DEPLOYMENT_TARGET=11.0 "$(RUSTUP_CARGO)" build --release --target aarch64-apple-darwin
	./packaging/check-macos-minos.sh target/aarch64-apple-darwin/release/qrbin 11.0

doctor:
	@echo "make uses cargo: $(RUSTUP_CARGO)"
	@echo "make uses rustc: $(RUSTC)"
	@"$(RUSTC)" --version
	@echo "Homebrew cargo, not used: /usr/local/bin/cargo"
	@/usr/local/bin/cargo --version
	@echo "Intel deployment target: $(MACOSX_DEPLOYMENT_TARGET)"
	@if [ -S /var/run/docker.sock ]; then \
		echo "docker: daemon socket present"; \
	else \
		echo "docker: client only, no daemon. make deb cannot run."; \
	fi

# Linux packaging needs a running Docker daemon. This Mac currently has the
# client and no socket, so these targets cannot run here.
deb linux:
	./packaging/build-linux.sh
