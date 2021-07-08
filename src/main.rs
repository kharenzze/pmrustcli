use bson::{bson};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct AppConfig {
    token: String,
}

fn main() {
    println!("Hello, world!");
    let a = bson!({
        "token": "tokenvalue",
        "tokewen": "tokenval"
    });

    let parsed: AppConfig = bson::from_bson(a).unwrap();
    println!("{}", &parsed.token);
}
