FROM rust:latest
MAINTAINER Emily Maré (emileet) <emileet@plsnobully.me>

WORKDIR /app

COPY data/presence.json /app/data/
COPY Cargo.lock Cargo.toml /app/
COPY src /app/src

RUN cargo install --path .

ENV TOKEN=VALUE

VOLUME ["/app/data"]
CMD ["discordsc"]