use dirs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{BufWriter, BufReader, Write};
use serde::{Serialize, Deserialize};
use bson::{Bson, Document};

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
        let file = file.unwrap();
        let mut buf = BufReader::new(file);
        let doc = Document::from_reader(&mut buf)
            .expect("error reading doc");

        bson::from_document(doc).unwrap_or(Self::empty())
    }

    pub fn save(&self) {
        let path = Self::get_file_path();
        let file: Result<File, std::io::Error> = {
            let f = OpenOptions::new()
            .read(false)
            .write(true)
            .truncate(true)
            .open(&path);
            if f.is_ok() {
                f
            } else {
                File::create(&path)
            }
        };
        if file.is_err() {
            panic!("Cannot open config file");
        }
        let file = file.unwrap();
        let redacted_bson = bson::to_bson(self).unwrap();
        let doc: Document = match redacted_bson {
            Bson::Document(d) => d,
            _ => panic!("bsom must be a document")
        };
        let mut buffer = BufWriter::new(file);
        doc.to_writer(&mut buffer).expect("error writing buffer");
        buffer.flush().expect("error writing buffer");
    }
}
