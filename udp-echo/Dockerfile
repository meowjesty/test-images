FROM rust:1.71 as builder

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/udp-echo /usr/local/bin/udp-echo
ENV RUST_LOG=debug

CMD ["udp-echo"]
