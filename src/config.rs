use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StatusDetails {
    pub text: String,
    pub emoji_id: Option<String>,
    pub emoji_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub token: String,
    pub delay: u64,
    pub statuses: Vec<StatusDetails>,
    pub display_names: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        let config_path = match std::env::var("HOME") {
            Ok(res) => res + "/.config/discordsc/config.json",
            Err(_) => "data/config.json".to_string(),
        };

        // attempt to read an existing config file, otherwise create an example
        let data = std::fs::read_to_string(&config_path)
            .or_else(|_| std::fs::read_to_string("data/config.json"))
            .map_err(|_| {
                std::fs::create_dir_all(std::path::Path::new(&config_path).parent().unwrap())
                    .expect(&format!(
                        "error: failed to create parent directories for '{}'!",
                        config_path
                    ));

                let example_config = serde_json::to_string_pretty(&AppConfig {
                    token: String::new(),
                    delay: 15,
                    statuses: vec![
                        StatusDetails {
                            text: "nyarch btw".to_string(),
                            emoji_id: Some("998888851386945566".to_string()),
                            emoji_name: "nyarch".to_string(),
                        },
                        StatusDetails {
                            text: "sakura".to_string(),
                            emoji_id: None,
                            emoji_name: "🌸".to_string(),
                        },
                    ],
                    display_names: vec!["emileet 🌸".to_string()],
                })
                .expect(&format!("error: failed to serialize example config!"));

                std::fs::write(&config_path, example_config)
                    .expect(&format!("error: failed to write to '{}'!", config_path));

                format!("error: please configure '{}'!", config_path)
            })
            .unwrap();

        serde_json::from_str(data.as_str())
            .expect("error: failed to deserialize file 'config.json'!")
    }
}
