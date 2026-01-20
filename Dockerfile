FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    git \
    openssl-dev

COPY Cargo.toml ./
COPY src ./src
COPY application.properties ./

RUN cargo install --path . --root /build

FROM alpine:latest AS flutter_builder

WORKDIR /flutter_app

RUN apk add --no-cache \
    bash \
    ca-certificates \
    curl \
    git \
    unzip \
    xz \
    libc6-compat \
    gcompat \
    libstdc++ \
    libgcc \
    gnome-keyring

RUN git clone --single-branch --branch master https://github.com/flutter/flutter.git /flutter
ENV PATH="/flutter/bin:${PATH}"

RUN flutter config --no-analytics

COPY flutter ./

RUN flutter build web --release --wasm

FROM alpine:latest

WORKDIR /app

COPY --from=builder /build/bin/Lunara /usr/local/bin/Lunara
COPY application.properties /app/application.properties
COPY --from=flutter_builder /flutter_app/build/web /app/flutter/build/web

CMD ["Lunara"]