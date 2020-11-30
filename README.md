# discordsc-rs
[![pipeline status](https://git.plsnobully.me/emileet/discordsc-rs/badges/master/pipeline.svg)](https://git.plsnobully.me/emileet/discordsc-rs/-/commits/master)

discord status changer - a simple rust app to automatically cycle through predefined custom statuses

## instructions

clone this repo and then build the application

```shell
cargo build
```

copy `example.env` to `.env` and specify your discord auth token
```shell
TOKEN=mfa.xxx
```

copy `data/example.json` to `data/presence.json` and configure it with your desired statuses
```json
{
    "statuses": [
        "emiyeet",
        "arch btw"
    ],
    "emojis": [
        {
            "emoji_id": "670569765034655774",
            "emoji_name": "purple-heart"
        },
        {
            "emoji_id": "670569765034655774",
            "emoji_name": "purple-heart"
        }
    ]
}
```

now run the application
```shell
cargo run
```

## docker

clone this repo and then build an image (ensure `data/presence.json` exists)

```shell
docker build -t emileet/discordsc-rs .
```

now spin up a container
```shell
docker run --detach \
  -e TOKEN=mfa.xxx \
  -v /host/data:/app/data \
  --name discordsc \
  emileet/discordsc-rs:latest
```