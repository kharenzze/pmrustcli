use dirs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub token: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let home_path = dirs::home_dir();
        if home_path.is_none() {
            panic!("Cannot find home path");
        }
        let home_path = home_path.unwrap();
        println!("{}", home_path.to_str().unwrap());
        println!("hi!");
        AppConfig {
            token: "asd".to_string()
        }
    }
}
