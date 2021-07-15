use std::fmt;
use serde_json::{Result as JSON_Result};
use serde::Deserialize;

type JSON = serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct SimpleItem {
  name: String,
  quadrant: u8,
  id: u64,
}

impl SimpleItem {
  pub fn empty() -> Self {
    SimpleItem {
      name: "".to_string(),
      quadrant: 0,
      id: 0,
    }
  }

  pub fn from_JSON(json: JSON) -> JSON_Result<Self> {
    let i: Self = serde_json::from_value(json)?;
    Ok(i)
  }
}

impl fmt::Display for SimpleItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", &self.name)
  }
}