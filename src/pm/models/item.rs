use std::fmt;
use serde_json::{Result as JSON_Result};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::Deserialize;
use chrono::{DateTime, Local};
use std::time::{UNIX_EPOCH, Duration};
use crate::{PM_BASE};

type JSON = serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Timestamp {
  String(String),
  Integer(u64),
  Float(f64),
}

impl From<&Timestamp> for u64 {
  fn from(item: &Timestamp) -> Self {
    match item {
      Timestamp::Float(f) => *f as u64,
      Timestamp::String(s) => s.parse().unwrap(),
      Timestamp::Integer(i) => *i,
    }
  }
}

impl From<&Timestamp> for DateTime<Local> {
  fn from(item: &Timestamp) -> Self {
    let parsed: u64 = item.into();
    secs_timestamp_to_datetime(parsed)
  }
}

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
  index: i64,
  state: ItemState,
  owner: Option<String>,
  owner_username: Option<String>,
  creator_username: Option<String>,
  icon: String,
  #[serde(rename = "completionPercentage")]
  completion_percentage: u8,
  #[serde(rename = "dueDate")]
  due_date: Option<Timestamp>,
  #[serde(rename = "startDate")]
  start_date: Option<Timestamp>,
  #[serde(rename = "completionDate")]
  completion_date: Option<Timestamp>,
  #[serde(rename = "creationDate")]
  creation_date: Timestamp,
  timestamp: Timestamp,
  #[serde(rename = "lastModifiedTimestamp")]
  last_modified_timestamp: Timestamp,
  #[serde(rename = "allDay")]
  all_day: bool,
  projects: Option<Vec<String>>,
}

fn secs_timestamp_to_datetime(timestamp: u64) -> DateTime<Local> {
  let d = UNIX_EPOCH + Duration::from_secs(timestamp);
  let dt = DateTime::<Local>::from(d);
  dt
}

impl SimpleItem {
  fn get_link(&self) -> String {
    let mut link = PM_BASE!("/office365/app/index/item/").to_string();
    link.push_str(&self.id.to_string());
    link
  }

  pub fn get_start_datetime(&self) -> Option<DateTime<Local>> {
    let t = self.start_date.as_ref()?;
    Some(t.into())
  }

  pub fn get_due_datetime(&self) -> Option<DateTime<Local>> {
    let t = self.due_date.as_ref()?;
    Some(t.into())
  }

  pub fn get_creation_datetime(&self) ->DateTime<Local> {
    let ref t = self.creation_date;
    t.into()
  }

  pub fn get_last_modified_datetime(&self) -> DateTime<Local> {
    let ref t = self.last_modified_timestamp;
    t.into()
  }

  pub fn from_json(json: JSON) -> JSON_Result<Self> {
    let i: Self = serde_json::from_value(json)?;
    Ok(i)
  }

  pub fn detailed_print(&self) {
    println!("{}", &self.name);
    println!("{}% completed", &self.completion_percentage);
    println!("Quadrant {}", &self.quadrant);
    if self.creator_username.is_some() {
      let name = self.creator_username.as_ref().unwrap();
      println!("Created by {}", name);
    }
    println!("Created {}", &self.get_creation_datetime());
    println!("Modified {}", &self.get_last_modified_datetime());
    if let Some(due) = self.get_due_datetime() {
      if let Some(start) = self.get_start_datetime() {
        println!("Starts {}", &start);
      }
      println!("Due {}", &due);
    }
    println!("Link {}", &self.get_link());
  }
}

impl fmt::Display for SimpleItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}\t{}", &self.get_link(), &self.name)
  }
}