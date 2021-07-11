use http::{HeaderMap, HeaderValue};
pub struct PMRest {
    client: reqwest::Client
}

type PMRestResult = Result<(), Box<dyn std::error::Error>>;

impl PMRest {
    fn get_headers(token: &String) -> HeaderMap {
        if token.len() == 0 {
            panic!("There's no token");
        }
        let mut h = HeaderMap::new();
        let auth_value = format!("Bearer {}", token);
        h.append("Authorization", HeaderValue::from_str(auth_value.as_str()).expect("Could not convert token"));
        h
    }

    pub fn new(token: &String) -> Self {
        let headers = Self::get_headers(token);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Cannot build HTTP client");

        PMRest {
            client
        }
    }
    pub fn get_me(&self) {

    }
}