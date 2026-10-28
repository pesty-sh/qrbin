FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/usr/local/bin:${PATH}

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl xz-utils python3 \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path

ARG ZIG_VERSION=0.16.0
RUN arch="$(uname -m)" \
    && curl -fsSL "https://ziglang.org/download/${ZIG_VERSION}/zig-${arch}-linux-${ZIG_VERSION}.tar.xz" \
    | tar -xJ -C /usr/local \
    && ln -s "/usr/local/zig-${arch}-linux-${ZIG_VERSION}/zig" /usr/local/bin/zig \
    && rustup target add x86_64-unknown-linux-gnu \
    && cargo install --locked cargo-zigbuild \
    && chmod -R a+rwX "${RUSTUP_HOME}" "${CARGO_HOME}"
