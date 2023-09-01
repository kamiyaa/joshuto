FROM ubuntu:latest

ARG TARGET

COPY ./target/${TARGET}/release/joshuto /usr/local/bin/joshuto
COPY ./config/ /root/.config/joshuto/

WORKDIR /root

ENTRYPOINT ["/usr/local/bin/joshuto"]
