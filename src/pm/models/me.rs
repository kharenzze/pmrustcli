use std::fmt;

type JSON = serde_json::Value;

pub struct Me(pub JSON); 

impl Me {
  pub fn get_email(&self) -> &str {
    self.0.get("email")
      .unwrap()
      .as_str()
      .unwrap()
  }
}

impl fmt::Display for Me {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let email = self.get_email();
    write!(f, "{}", email)
  }
}