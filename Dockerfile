# Stage 1: Build
FROM --platform=linux/amd64 rust:1.91-alpine AS builder
WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
  musl-dev \
  pkgconfig \
  openssl-dev \
  openssl-libs-static \
  protobuf-dev \
  protoc

# Copy source code
COPY . .

# Build release binary
RUN rustup target add x86_64-unknown-linux-musl && \
  cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS export
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/grpc_client /

