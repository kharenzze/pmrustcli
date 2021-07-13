use std::fmt;

type JSON = serde_json::Value;

pub struct Me(pub JSON); 

impl fmt::Display for Me {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let email: &str = self.0.get("email")
      .unwrap()
      .as_str()
      .unwrap();
    write!(f, "{}", email)
  }
}