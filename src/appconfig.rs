use dirs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use serde::{Serialize, Deserialize};
//use bson::

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

    pub fn save(&self) {
        let path = Self::get_file_path();
        let file = OpenOptions::new().read(false).write(true).open(path);
        if file.is_err() {
            panic!("Cannot open config file");
        }
        let file = file.unwrap();
        let redacted_bson = bson::to_bson(self).unwrap();
        let mut buffer = BufWriter::new(file);
        write!(&mut buffer, "{:?}", &redacted_bson);
        //buffer.write_all(redacted_bson.)
    }
}
