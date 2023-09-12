# docker build -t joshuto:local-dev -f utils/docker/builder.Dockerfile \
#   --build-arg TARGET="x86_64-unknown-linux-musl" \
#   --build-arg RELEASE_I="scratch" .

ARG RELEASE_I="busybox:latest"
ARG TARGET="x86_64-unknown-linux-musl"

FROM rust:latest AS builder

ARG TARGET

COPY . /usr/src/joshuto

WORKDIR /usr/src/joshuto

RUN rustup target add ${TARGET} && rustup target add ${TARGET}
RUN cargo build --target ${TARGET} --release

FROM ${RELEASE_I} AS release

ARG TARGET

COPY --from=builder /usr/src/joshuto/target/${TARGET}/release/joshuto /usr/local/bin/joshuto
COPY ./config/ /root/.config/joshuto/

WORKDIR /root

ENTRYPOINT ["/usr/local/bin/joshuto"]
