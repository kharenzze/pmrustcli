use http::{HeaderMap, HeaderValue};
pub struct PMRest {
    client: reqwest::Client
}

type JSON = serde_json::Value;

enum PMResponse {
    Me(JSON) 
}

type PMRestResult = Result<PMResponse, Box<dyn std::error::Error>>;
type PMRestResultUnwrapped = Result<JSON, Box<dyn std::error::Error>>;

macro_rules! PM_BASE {
    ($path:expr) => {
        concat!("https://sync.appfluence.com", $path);
    };
}

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

    pub async fn get_me(&self) -> PMRestResult {
        let json: JSON = self.execute_request(PM_BASE!("/api/v1/me")).await?;
        Ok(PMResponse::Me(json))
    }

    async fn execute_request(&self, url: &str) -> PMRestResultUnwrapped {
        let json: JSON = self.client.get(url)
            .send()
            .await?
            .json()
            .await?;
        Ok(json)
    }
}