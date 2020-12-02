FROM alpine:latest
MAINTAINER Emily Maré (emileet) <emileet@plsnobully.me>

RUN wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.32-r0/glibc-2.32-r0.apk && \
    wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub && \
    apk add --no-cache -U glibc-2.32-r0.apk libgcc

WORKDIR /app

COPY data/config.json /app/data/
COPY discordsc /app/run

ENV LD_LIBRARY_PATH=/lib:/usr/lib:$LD_LIBRARY_PATH
ENV TOKEN=VALUE

VOLUME ["/app/data"]
CMD ["/app/run"]