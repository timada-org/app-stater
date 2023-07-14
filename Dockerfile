# syntax=docker/dockerfile:1

FROM rust:1.70-alpine3.17 AS chef

RUN apk add --no-cache musl-dev tzdata \
        openssl-dev openssl-libs-static \
        pkgconf git libpq-dev \
        protoc protobuf-dev

ENV USER=starter
ENV UID=10001

# See https://stackoverflow.com/a/55757473/12429735
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Set `SYSROOT` to a dummy path (default is /usr) because pkg-config-rs *always*
# links those located in that path dynamically but we want static linking, c.f.
# https://github.com/rust-lang/pkg-config-rs/blob/54325785816695df031cef3b26b6a9a203bbc01b/src/lib.rs#L613
ENV SYSROOT=/dummy

# The env var tells pkg-config-rs to statically link libpq.
ENV LIBPQ_STATIC=1

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-chef
RUN cargo install cargo-leptos

RUN mkdir -p ~/.cache/cargo-leptos/wasm-opt-version_112/wasm-opt-version_112
RUN wget -qO- https://github.com/WebAssembly/binaryen/releases/download/version_112/binaryen-version_112-x86_64-linux.tar.gz | tar xvz -C ~/.cache/cargo-leptos/wasm-opt-version_112/wasm-opt-version_112

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef prepare --recipe-path recipe-ssr.json
RUN cargo chef prepare --recipe-path recipe-hydrate.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
COPY --from=planner /app/recipe.json recipe-ssr.json
COPY --from=planner /app/recipe.json recipe-hydrate.json

# Build dependencies - this is the caching Docker layer!

RUN cargo chef cook --release --package=starter-cli --recipe-path recipe.json
RUN cargo chef cook --release --no-default-features --features=ssr --target-dir=target/server --package=starter-app --recipe-path recipe-ssr.json
RUN cargo chef cook --release --no-default-features --features=hydrate --target-dir=target/front --target=wasm32-unknown-unknown --package=starter-app --recipe-path recipe-hydrate.json

# Build application

COPY . .

RUN cargo build --release --bin starter-cli --package starter-cli
RUN cargo leptos build --release

FROM scratch
 ARG version=unknown
 ARG release=unreleased
 LABEL name="Starter" \
       maintainer="info@timada.co" \
       vendor="Timada" \
       version=${version} \
       release=${release} \
       summary="High-level summary" \
       description="A bit more details about this specific container"

COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/target/server/release/starter-app /usr/bin/starter-server
COPY --from=builder /app/target/release/starter-cli /usr/bin/starter-cli
COPY --from=builder /app/target/site /etc/starter/site

USER starter:starter

ENV LEPTOS_SITE_ROOT=/etc/starter/site
ENV LEPTOS_SITE_PKG_DIR=starter-pkg

EXPOSE 3000 4000

ENTRYPOINT [ "starter-cli" ]
CMD ["serve"]

