VERSION 0.7
FROM alpine

build:
    BUILD --platform linux/amd64 +docker-image
    BUILD --platform linux/arm64 +docker-image

lint:
    BUILD +cargo-clippy
    BUILD +hadolint
    BUILD +renovate-validate

docker-image:
    ARG TARGETPLATFORM
    FROM DOCKERFILE \
        -f Dockerfile \
        --platform $TARGETPLATFORM .

hadolint:
    # renovate: datasource=docker depName=hadolint/hadolint versioning=docker
    ARG HADOLINT_VERSION=2.12.0-alpine
    FROM hadolint/hadolint:$HADOLINT_VERSION
    WORKDIR /images
    COPY .hadolint.yaml .
    COPY Dockerfile .
    RUN hadolint Dockerfile

rust-app:
    WORKDIR ~
    COPY Cargo.lock .
    COPY Cargo.toml .
    COPY --dir src/ .
    SAVE ARTIFACT . /src

rust-env:
    # renovate: datasource=docker depName=rust versioning=docker
    ARG RUST_VERSION=1.82.0
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
    ARG RENOVATE_VERSION=39
    FROM renovate/renovate:$RENOVATE_VERSION
    WORKDIR /usr/src/app
    COPY renovate.json .
    RUN renovate-config-validator
