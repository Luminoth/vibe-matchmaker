FROM rust:1.77-slim AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin engine

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/engine /usr/local/bin/engine

ENV RUST_LOG=info

CMD ["engine"]
