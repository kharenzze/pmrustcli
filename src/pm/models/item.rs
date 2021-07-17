use std::fmt;
use serde_json::{Result as JSON_Result};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::Deserialize;
use crate::{PM_BASE};

type JSON = serde_json::Value;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum ItemState {
  UNFINISHED = 0,
  DONE,
  TRASHED,
  DELETED,
}

#[derive(Deserialize, Debug)]
pub struct SimpleItem {
  name: String,
  quadrant: u8,
  id: u64,
  idd: u64,
  index: u64,
  state: ItemState,
  owner: Option<String>,
  owner_username: Option<String>,
  creator_username: Option<String>,
  icon: String,
  #[serde(rename = "completionPercentage")]
  completion_percentage: u8,
  #[serde(rename = "dueDate")]
  due_date: Option<u64>,
  #[serde(rename = "startDate")]
  start_date: Option<u64>,
  #[serde(rename = "completionDate")]
  completion_date: Option<u64>,
  #[serde(rename = "creationDate")]
  creation_date: u64,
  timestamp: u64,
  #[serde(rename = "lastModifiedTimestamp")]
  last_modified_timestamp: u64,
  #[serde(rename = "allDay")]
  all_day: bool,
  projects: Vec<String>,
}

impl SimpleItem {
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