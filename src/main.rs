mod config;
mod types;
mod utility;

use config::AppConfig;
use types::{CustomStatus, PatchName, PatchStatus};
use utility::{clamp, request};

fn main() {
    dotenv::dotenv().ok();

    println!("discordsc-rs - discord status changer");
    println!("-------------------------------------");

    let mut status_index: usize = 0;
    let mut name_index: usize = 0;

    loop {
        let config = AppConfig::load();

        if status_index >= config.statuses.len() {
            status_index = 0;
        }

        if name_index >= config.statuses.len() {
            name_index = 0;
        }

        if config.statuses.len() > 0 {
            request(
                "https://discord.com/api/v9/users/@me/settings",
                Some(config.token.clone()),
                serde_json::to_string(&PatchStatus {
                    custom_status: CustomStatus {
                        text: config.statuses[status_index].text.clone(),
                        emoji_id: config.statuses[status_index].emoji_id.clone(),
                        emoji_name: config.statuses[status_index].emoji_name.clone(),
                        expires_at: None,
                    },
                })
                .unwrap(),
            );

            println!("status: {}", config.statuses[status_index].text);
            status_index += 1;
        }

        if config.display_names.len() > 0 {
            request(
                "https://discord.com/api/v9/users/@me",
                Some(config.token.clone()),
                serde_json::to_string(&PatchName {
                    global_name: config.display_names[name_index].clone(),
                })
                .unwrap(),
            );

            println!("name: {}", config.display_names[name_index]);
            name_index += 1;
        }

        let delay = clamp(config.delay, 1, config.delay);
        std::thread::sleep(std::time::Duration::from_secs(delay));
    }
}
