FROM rust:latest AS builder

COPY . /usr/src/joshuto

WORKDIR /usr/src/joshuto

RUN cargo build --release

RUN rustup target add x86_64-unknown-linux-musl \
  && rustup target add aarch64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM busybox:latest

COPY --from=builder /usr/src/joshuto/target/x86_64-unknown-linux-musl/release/joshuto /usr/local/bin/joshuto
COPY ./config/ /root/.config/joshuto/

WORKDIR /root

ENTRYPOINT ["/usr/local/bin/joshuto"]
