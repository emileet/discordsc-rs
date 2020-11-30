FROM archlinux:latest
MAINTAINER Emily Maré (emileet) <emileet@plsnobully.me>

WORKDIR /app

COPY data/presence.json /app/data/
COPY discordsc /app/run

ENV TOKEN=VALUE

VOLUME ["/app/data"]
CMD ["/app/run"]