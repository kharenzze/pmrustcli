use std::fmt;
use serde_json::{Result as JSON_Result};
use serde::Deserialize;
use crate::{PM_BASE};

type JSON = serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct SimpleItem {
  name: String,
  quadrant: u8,
  id: u64,
}

impl SimpleItem {
  pub fn _empty() -> Self {
    SimpleItem {
      name: "".to_string(),
      quadrant: 0,
      id: 0,
    }
  }

  fn get_link(&self) -> String {
    let mut link = PM_BASE!("/office365/app/index/item/").to_string();
    link.push_str(&self.id.to_string());
    link
  }

  pub fn from_json(json: JSON) -> JSON_Result<Self> {
    let i: Self = serde_json::from_value(json)?;
    Ok(i)
  }
}

impl fmt::Display for SimpleItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}\t{}", &self.name, &self.get_link())
  }
}