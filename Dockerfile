FROM rust:latest AS builder

COPY . /usr/src/joshuto

WORKDIR /usr/src/joshuto

RUN rustup target add x86_64-unknown-linux-musl \
  && cargo build --target x86_64-unknown-linux-musl --release

FROM busybox:latest

COPY --from=builder /usr/src/joshuto/target/x86_64-unknown-linux-musl/release/joshuto /bin/joshuto

WORKDIR /root

