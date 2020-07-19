//! # RusTOA
//!
//! `rustoa` is a crate you can use to access The Orange Alliance API.
//! This crate makes it easy to access the official First Tech Challenge API
//! and use it in your Rust projects.

use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;
use reqwest::blocking::Response;

/// The main RusTOA client.
///
/// You can use the client to get the API version.
#[derive(Clone)]
pub struct Client {
    api_key: String,
    application_name: String,
}

impl Client {
    #[doc(hidden)]
    pub fn request(&self, target: &str) -> Result<Response, Box<dyn std::error::Error>> {
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

    /// Create a new Client object.
    /// The only parameter this method takes is your Orange Alliance API key as a `String`.
    /// It returns a Client object.
    pub fn new(api_key: String) -> Client {
        Client {
            api_key,
            application_name: "rustoa".to_string()
        }
    }

    /// Get the version of The Orange Alliance API that this crate is using.
    /// This method takes no arguments and returns the version as a String.
    ///
    /// # Panics
    /// This method can panic in three ways:
    /// - The HTTP request to the API fails. This can be because the API is either down or you are
    /// being ratelimited.
    /// - Serde cannot properly deserialize the JSON data in the response. This happens because the
    /// API has sent invalid JSON.
    /// - The HashMap does not have the needed keys to process the data. This happens because
    /// the request was made to the wrong target or the API has sent back an error in JSON form.
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

#[cfg(test)]
mod tests {
    fn create_client() -> super::Client {
        super::Client::new("1e48fa3b34a8ab86cbec44735c5b6055a141f245455faac878bfa204e35c1a7e".to_string())
    }
    #[test]
    fn correct_version() {
        let client = create_client();
        assert_eq!("3.7.0", client.api_version());
    }
}