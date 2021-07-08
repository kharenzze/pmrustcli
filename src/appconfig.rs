use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub token: String,
}

impl AppConfig {
    pub fn load() -> Self {
        println!("hi!");
        AppConfig {
            token: "asd".to_string()
        }
    }
}

