use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;
use reqwest::blocking::Response;


pub struct Client {
    api_key: String,
    application_name: String,
}

impl Client {
    fn request(&self, target: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("https://theorangealliance.org/api{}", target);
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url[..])
            .header("X-TOA-Key", &self.api_key)
            .header("X-Application-Origin", &self.application_name)
            .header(CONTENT_TYPE, "application/json")
            .send()?;

        Ok(resp)
    }
    pub fn new(api_key: String) -> Client {
        Client {
            api_key,
            application_name: "rustoa".to_string()
        }
    }
    pub fn api_version(&self) -> String {
        let resp = match self.request("/") {
            Ok(resp) => resp,
            Err(e) => {
                panic!("Something went wrong: {}", e);
            }
        };

        let map = match resp.json::<HashMap<String, String>>() {
            Ok(m) => m,
            Err(e) => panic!("Something went wrong: {}", e)
        };

        match map.get("version") {
            Some(vers) => vers.to_string(),
            None => panic!("Something went wrong with the API.")
        }
    }
}