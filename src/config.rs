use dirs;

use crate::action::Action;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub actions: Vec<Action>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .map(|dir| dir.join("roki").join("config.yaml"))
            .unwrap();

        println!("config_path: {:?}", config_path);

        if config_path.exists() {
            let config = std::fs::read_to_string(config_path).unwrap();
            serde_yaml::from_str(&config).expect("config load error")
        } else {
            Self {
                actions: Vec::new(),
            }
        }
    }
}
