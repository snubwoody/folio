FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
RUN apt update
RUN apt install -y libwebkit2gtk-4.1-dev \
      build-essential

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin folio-worker

# We do not need the Rust toolchain to run the binary!
FROM debian AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    ca-certificates 
COPY --from=builder /app/target/release/folio-worker /usr/local/bin
ENTRYPOINT ["/usr/local/bin/folio-worker"]
