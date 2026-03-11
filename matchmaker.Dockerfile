FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin matchmaker

FROM debian:trixie-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY etc/ etc/
COPY --from=builder /app/target/release/matchmaker /usr/local/bin/matchmaker

ENV RUST_LOG=info

CMD ["matchmaker"]
