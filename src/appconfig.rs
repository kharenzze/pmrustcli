use dirs;
use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub token: String,
}

impl AppConfig {
    fn empty() -> Self {
        AppConfig {
            token: "".to_string()
        }
    }

    pub fn load() -> Self {
        let home_path = dirs::home_dir();
        if home_path.is_none() {
            panic!("Cannot find home path");
        }
        let mut home_path = home_path.unwrap();
        home_path.push(".pmcli");
        let file = File::open(home_path);
        if file.is_err() {
            return Self::empty()
        }
        AppConfig {
            token: "asd".to_string()
        }
    }
}
