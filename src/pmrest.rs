pub struct PMRest {
    client: reqwest::Client
}

type PMRestResult = Result<(), Box<dyn std::error::Error>>;

impl PMRest {
    pub fn get_me(&self) {

    }
}