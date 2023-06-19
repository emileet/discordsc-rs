# discordsc-rs

[![pipeline status](https://git.plsnobully.me/emileet/discordsc-rs/badges/master/pipeline.svg)](https://git.plsnobully.me/emileet/discordsc-rs/-/commits/master)

discord status changer - a simple rust app to automatically cycle through predefined custom statuses and display names

## instructions

clone this repo and then build the application

```shell
cargo build --release
```

copy `data/example.json` to `data/config.json` and configure it to your desires

```json
{
  "token": "emi1337",
  "delay": 15,
  "statuses": [
    {
      "text": "nyarch btw",
      "emoji_id": "998888851386945566",
      "emoji_name": "nyarch"
    },
    {
      "text": "sakura",
      "emoji_id": null,
      "emoji_name": "🌸"
    }
  ],
  "display_names": []
}
```

now run the application

```shell
cargo run --release
```

## docker

clone this repo and then build an image (ensure `data/config.json` exists)

```shell
docker build -t emileet/discordsc-rs .
```

alternatively, pull the image from the pod.plsnobully.me container registry

```shell
docker pull pod.plsnobully.me/emileet/discordsc-rs:latest
```

now spin up a container (if using volumes, please refer to [notes](#notes))

```shell
docker run --detach \
  -e TOKEN=emi1337 \
  -v /host/data:/app/data \
  --name discordsc \
  emileet/discordsc-rs:latest
```

## notes

- one way to retrieve a discord token is to open the developer console (ctrl+shift+i) in the discord client, go to the network tab, navigate to a discord server and under the **Name** pane look for something like `messages?limit=50`; where you'll find under the **Headers** tab the token in the **Request Headers** as `authorization: 0xbabe.emi1337`
- it's possible to specify a discord token using the **TOKEN** env var, including providing a `.env` file, thus allowing for it to be omitted from `config.json`
- container deployments should ensure that their volume points to either `/app/data` or `/root/.config/discordsc`
