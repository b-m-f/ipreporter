FROM docker.io/library/rust:1.67 as builder

RUN apt-get update && apt-get install -yy musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --target-dir .

FROM alpine:latest

WORKDIR /usr/bin/
COPY --from=builder /app/x86_64-unknown-linux-musl/release/ipreporter .

RUN chmod +x /usr/bin/ipreporter
CMD ["ipreporter"]

