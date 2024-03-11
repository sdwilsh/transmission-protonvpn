VERSION 0.7
FROM alpine

lint:
    BUILD +cargo-clippy
    BUILD +renovate-validate

rust-app:
    WORKDIR ~
    COPY Cargo.lock .
    COPY Cargo.toml .
    COPY --dir src/ .
    SAVE ARTIFACT . /src

rust-env:
    # renovate: datasource=docker depName=rust versioning=docker
    ARG RUST_VERSION=1.76.0
    FROM rust:$RUST_VERSION
    WORKDIR /usr/src/myapp
    COPY rust-toolchain .

cargo-clippy:
    FROM +rust-env
    WORKDIR /usr/src/myapp
    RUN rustup component add clippy
    COPY +rust-app/src /usr/src/myapp
    RUN cargo clippy --all-targets --all-features -- -D warnings

renovate-validate:
    # renovate: datasource=docker depName=renovate/renovate versioning=docker
    ARG RENOVATE_VERSION=37
    FROM renovate/renovate:$RENOVATE_VERSION
    WORKDIR /usr/src/app
    COPY renovate.json .
    RUN renovate-config-validator
