fn main() {
    // load .env
    dotenv::dotenv().ok();

    println!("discordsc-rs - discord status changer");
    println!("-------------------------------------");

    // index of current status
    let mut status_index: usize = 0;

    // main logic
    loop {
        // load and then parse the configuration file
        let data = json::parse(load_configuration().as_str())
            .expect("error: failed to parse file 'config.json'!");

        // sanity check for status_index
        if status_index >= data["statuses"].len() {
            status_index = 0;
        }

        // construct request
        let request = reqwest::blocking::Client::new()
            .patch("https://discordapp.com/api/v6/users/@me/settings")
            .header(
                reqwest::header::AUTHORIZATION,
                // retrieve token from env var
                match std::env::var("TOKEN") {
                    // retrieve token from config.json if the former doesn't exist
                    Err(_) => match data["token"].is_empty() {
                        true => panic!("error: a discord token was not specified!"),
                        false => data["token"].to_string(),
                    },
                    Ok(res) => res,
                },
            )
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json::stringify(json::object! {
                custom_status: {
                    text: data["statuses"][status_index]["text"].as_str().unwrap(),
                    emoji_id: data["statuses"][status_index]["emoji_id"].as_str().unwrap(),
                    emoji_name: data["statuses"][status_index]["emoji_name"].as_str().unwrap(),
                    expires_at: null
                }
            }));

        // send request
        request.send().expect("error: unable to send request!");

        // print status to console
        println!("status: {}", data["statuses"][status_index]["text"]);

        // sleep for the configured amount of time
        match data["delay"].is_number() {
            false => panic!("error: a delay time was not specified!"),
            true => {
                let delay = data["delay"].as_u64().unwrap();
                std::thread::sleep(std::time::Duration::from_secs(clamp(delay, 1, delay)));
            }
        };

        // increment status index
        status_index += 1;
    }
}

fn load_configuration() -> String {
    // load from ~/.config/discordsc/config.json
    let config_path = match std::env::var("HOME") {
        Err(_) => String::from("data/config.json"),
        Ok(res) => res + "/.config/discordsc/config.json",
    };

    return match std::fs::read_to_string(&config_path) {
        // load from ./data/config.json if the former fails
        Err(_) => match std::fs::read_to_string("data/config.json") {
            // create configuration when both fail
            Err(_) => {
                // construct example configuration
                let config_file = json::object! {
                    token: "mfa.emi1337",
                    delay: 15,
                    statuses: [
                        {
                            text: "emileet is the best",
                            emoji_id: "670569765034655774",
                            emoji_name: "purpleheart"
                        },
                        {
                            text: "i'm a dummiehead bully",
                            emoji_id: "670569765034655774",
                            emoji_name: "purpleheart"
                        }
                    ]
                };

                // create parent directories
                std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap())
                    .expect(&format!(
                        "error: failed to create parent directories for '{}'!",
                        config_path
                    ));

                // write example configuration
                config_file
                    .write_pretty(
                        &mut std::fs::File::create(&config_path)
                            .expect(&format!("error: failed to create '{}'!", config_path)),
                        4,
                    )
                    .expect(&format!("error: failed to write to '{}'!", config_path));

                // inform the user that intervention is required
                panic!("error: please configure '{}'!", config_path);
            }
            Ok(res) => res,
        },
        Ok(res) => res,
    };
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
