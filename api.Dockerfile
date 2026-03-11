FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin api

FROM debian:trixie-slim
WORKDIR /app

# Install native dependencies if needed (e.g., OpenSSL)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY etc/ etc/
COPY --from=builder /app/target/release/api /usr/local/bin/api

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080
CMD ["api"]
