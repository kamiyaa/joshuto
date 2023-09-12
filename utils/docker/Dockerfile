# For images like alpine, busybox, scratch with no libc you can use musl targets instead
#
# docker build -t joshuto:local-dev -f utils/docker/Dockerfile \
#   --build-arg TARGET="x86_64-unknown-linux-musl" \
#   --build-arg RELEASE_I="scratch" .

ARG RELEASE_I="ubuntu:lunar"
ARG TARGET

FROM ${RELEASE_I}

ARG TARGET

COPY ./target/${TARGET}/release/joshuto /usr/local/bin/joshuto
COPY ./config/ /root/.config/joshuto/

WORKDIR /root

ENTRYPOINT ["/usr/local/bin/joshuto"]
