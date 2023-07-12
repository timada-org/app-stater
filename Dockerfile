FROM rust:1.70-alpine3.17 AS chef

RUN apk add --no-cache musl-dev tzdata \
        openssl-dev openssl-libs-static \
        pkgconf git libpq-dev \
        protoc protobuf-dev

RUN rustup toolchain install nightly
RUN rustup default nightly

RUN cargo install cargo-chef
RUN cargo install cargo-leptos
#RUN cargo install wasm-opt
RUN rustup target add wasm32-unknown-unknown

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

ENV LEPTOS_SITE_PKG_DIR=pkg

# Build dependencies - this is the caching Docker layer!

RUN cargo chef cook --release --package=starter-cli --recipe-path recipe.json
RUN cargo chef cook --release --features=ssr --target-dir=target/server --package=starter-app --recipe-path recipe-ssr.json
RUN cargo chef cook --release --features=hydrate --target-dir=target/front --target=wasm32-unknown-unknown --package=starter-app --recipe-path recipe-hydrate.json

# Build application

COPY . .

RUN cargo build --release --bin starter-cli --package starter-cli
RUN cargo leptos build --release

