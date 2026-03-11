FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin allocator

FROM debian:trixie-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY etc/ etc/
COPY --from=builder /app/target/release/allocator /usr/local/bin/allocator

ENV RUST_LOG=info

CMD ["allocator"]
