FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libavfilter-dev \
    libavdevice-dev \
    libssl-dev \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs

# Cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Copy source and build for real
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Ensure the target directory exists for the volume mount
RUN mkdir -p /app/target/release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libavcodec59 \
    libavformat59 \
    libavutil57 \
    libswscale6 \
    libavfilter8 \
    libavdevice59 \
    libx11-6 \
    libssl3

WORKDIR /app
COPY --from=builder /app/target/release/citycam /app/citycam
COPY --from=builder /app/target/release/citycam /app/target/release/
CMD ["/app/citycam"]
