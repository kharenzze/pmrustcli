use dirs;
use std::{fs::File, path::PathBuf};
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

    fn get_file_path() -> PathBuf {
        let home_path = dirs::home_dir();
        if home_path.is_none() {
            panic!("Cannot find home path");
        }
        let mut home_path = home_path.unwrap();
        home_path.push(".pmcli");
        home_path
    }

    pub fn load() -> Self {
        let path = Self::get_file_path();
        let file = File::open(path);
        if file.is_err() {
            return Self::empty()
        }
        AppConfig {
            token: "asd".to_string()
        }
    }
}
