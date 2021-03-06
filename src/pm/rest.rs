use serde::Deserialize;
use std::fmt;
use serde::de::DeserializeOwned;
use serde_json::{Result as JSON_Result, json};
use urlencoding::encode;
use std::collections::HashMap;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Response, Error};
use crate::pm::models::me::{Me};
use crate::pm::models::item::{SimpleItem};
use crate::{PM_BASE};
pub struct PMRest {
    client: reqwest::Client
}

type JSON = serde_json::Value;
type DError = Box<dyn std::error::Error>;
type PMRestResultUnwrapped = Result<JSON, DError>;

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

    pub async fn get_me(&self) -> Result<Me, DError> {
        let json: JSON = self.execute_request(PM_BASE!("/api/v1/me")).await?;
        Ok(Me(json))
    }

    pub async fn get_item(&self, id: u64) -> Result<SimpleItem, DError> {
        let base = PM_BASE!("/api/v2/item_summary");
        let url = format!("{}/{}", base, id);
        let json: JSON = self.execute_request(&url).await?;
        let item = SimpleItem::from_json(json)?;
        Ok(item)
    }

    pub async fn get_alerts(&self) -> Result<PMObjectsResponse<SimpleItem>, DError> {
        let mut params = QueryParams::new();
        params.set("summaries", "1");
        let base = PM_BASE!("/api/v2/item/attention");
        let mut url: String = base.to_string();
        url.push_str(&params.to_string());
        let json: JSON = self.execute_request(&url).await?;
        let result: PMObjectsResponse<SimpleItem> = PMObjectsResponse::from_json(json)?;
        Ok(result)
    }

    pub async fn post_item(&self, text: String) -> Result<SimpleItem, DError> {
        let body = json!({
            "name": &text,
        });

        let url = PM_BASE!("/api/v1/item/");

        let res = self.client.post(url)
            .body(body.to_string())
            .header("content-type", HeaderValue::from_static("application/json"))
            .send()
            .await;

        Self::auth_check(&res);

        let json: JSON = res?
            .json()
            .await?;

        let item = SimpleItem::from_json(json)?;
        Ok(item)
    }

    pub async fn search(&self, text: &str) -> Result<PMObjectsResponse<SimpleItem>, DError> {
        let mut params = QueryParams::new();
        params.set("state_le", "0");
        params.set("summaries", "1");
        params.set("limit", "20");
        params.set("order_by", "-lastModifiedTimestamp");
        params.set("offset", "0");
        params.set("q", &encode(text).to_string());

        let params = params.to_string();
        let mut url = PM_BASE!("/api/v1/me/search").to_string();
        url.push_str(&params);
        println!("{}", url);

        let json: JSON = self.execute_request(&url).await?;
        let result: PMObjectsResponse<SimpleItem> = PMObjectsResponse::from_json(json)?;
        Ok(result)
    }

    async fn execute_request(&self, url: &str) -> PMRestResultUnwrapped {
        let res = self.client.get(url)
            .send()
            .await;

        Self::auth_check(&res);
        let res = res?;
        
        let json: JSON = res
            .json()
            .await?;
        Ok(json)
    }
    fn auth_check(response_result: &Result<Response, Error>) {
        if response_result.is_ok() {
            let res = response_result.as_ref().unwrap();
            let status = res.status();
            if status == StatusCode::UNAUTHORIZED {
                println!("{}", status);
                panic!();
            }
        }
    }
}

type QParams = HashMap<String, String>;
struct QueryParams {
    map: QParams
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            map: QParams::new()
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut r = "?".to_string();
        for (k ,v) in &self.map {
            r.push_str(&format!("{}={}&", k.as_str() , v.as_str()));
        }
        if r.len() != 1 {
            r.pop();
        } else {
            return "".to_string();
        }
        r
    }
}

#[derive(Deserialize, Debug)]
pub struct APIMeta {
    limit: usize,
    offset: usize,
    total_count: usize,
    requested_time: f64,
}

#[derive(Deserialize, Debug)]
pub struct PMObjectsResponse<T> {
    pub meta: APIMeta,
    pub objects: Vec<T>,
}

impl <T: DeserializeOwned> PMObjectsResponse<T> {
  pub fn from_json(json: JSON) -> JSON_Result<Self> {
    let i: Self = serde_json::from_value(json)?;
    Ok(i)
  }
}

struct AppError {} 

impl std::error::Error for AppError {}

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}