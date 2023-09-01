# FROM rust:latest AS builder

# COPY . /usr/src/joshuto

# WORKDIR /usr/src/joshuto

# RUN rustup target add x86_64-unknown-linux-musl \
#   && rustup target add aarch64-unknown-linux-musl \
#   && rustup target add x86_64-apple-darwi

# RUN cargo build --target x86_64-unknown-linux-musl --release
# RUN cargo build --target x86_64-apple-darwis --release

FROM busybox:latest

# COPY --from=builder /usr/src/joshuto/target/x86_64-unknown-linux-musl/release/joshuto /usr/local/bin/joshuto
COPY ./target/release/joshuto /usr/local/bin/joshuto
COPY ./config/ /root/.config/joshuto/

WORKDIR /root

ENTRYPOINT ["/usr/local/bin/joshuto"]
