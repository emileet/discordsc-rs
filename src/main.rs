use dotenv::dotenv;
use std::io::prelude::*;
use std::{thread, time};

fn main() {
    // load .env
    dotenv().ok();

    // handle graceful exits
    thread::spawn(|| {
        command_line();
    });

    // main logic
    let mut status_index: usize = 0;
    loop {
        // parse presence.json
        let data = json::parse(
            std::fs::read_to_string("data/presence.json")
                .expect("error: 'data/presence.json' doesn't exist!")
                .as_str(),
        )
        .expect("error: failed to parse 'data/presence.json'!");

        // sanity check for status_index
        if status_index >= data["statuses"].len() {
            status_index = 0;
        }

        // construct request
        let request = reqwest::blocking::Client::new()
            .patch("https://discordapp.com/api/v6/users/@me/settings")
            .header(
                reqwest::header::AUTHORIZATION,
                std::env::var("TOKEN").expect("error: 'TOKEN' environment variable not specified!"),
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json::stringify(json::object! {
                custom_status: {
                    text: data["statuses"][status_index].as_str().unwrap(),
                    emoji_id: data["emojis"][status_index]["emoji_id"].as_str().unwrap(),
                    emoji_name: data["emojis"][status_index]["emoji_name"].as_str().unwrap(),
                    expires_at: null
                }
            }));

        // send request
        request.send().expect("error: unable to send request!");

        // sleep for 15 secs and increment status_index
        thread::sleep(time::Duration::from_secs(15));
        status_index += 1;
    }
}

fn command_line() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error: failed to read line!");

        if input.starts_with("exit") {
            std::process::exit(0);
        }
    }
}
