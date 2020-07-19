//! # RusTOA
//!
//! `rustoa` is a crate you can use to access The Orange Alliance API.
//! This crate makes it easy to access the official First Tech Challenge API
//! and use it in your Rust projects.

use reqwest::blocking::Response;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

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
    #[doc(hidden)]
    pub fn api_key(&self) -> &str {
        &self.api_key[..]
    }
    #[doc(hidden)]
    pub fn application_name(&self) -> &str {
        &self.application_name[..]
    }

    /// Create a new Client object.
    /// # Arguments
    ///
    /// * `api_key` - Your Orange Alliance API key as a `String`.
    ///
    /// It returns a Client object.
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
            application_name: "rustoa".to_string(),
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
            Err(e) => panic!("Something went wrong: {}", e),
        };

        match map.get("version") {
            Some(vers) => vers.to_string(),
            None => panic!("Something went wrong with the API."),
        }
    }
    /// This method is used to get an instance of `Team`.
    /// # Arguments
    ///
    /// * `team_number` - The FTC team number as a `u32` integer.
    ///
    /// It returns a Team object with the necessary data
    pub fn team(&self, team_number: u32) -> Team {
        Team::new(team_number, self.clone())
    }
}

/// A struct used to access an FTC team.
///
/// Do not create this struct yourself. Instead use your `Client` instance.
pub struct Team {
    client: Client,
    pub team_number: u32,
}

impl Team {
    #[doc(hidden)]
    pub fn new(team_number: u32, client: Client) -> Team {
        Team {
            // api_key: client.api_key().to_string(),
            // application_name: client.application_name().to_string(),
            client,
            team_number,
        }
    }
    /// The total amount of times the team has won a match.
    ///
    /// This method takes no arguments.
    ///
    /// It returns a `u32` integer.
    pub fn wins(&self) -> u32 {
        let resp = match self
            .client
            .request(&format!("/team/{}/wlt", self.team_number)[..])
        {
            Ok(resp) => resp,
            Err(e) => panic!("Something went wrong: {}", e),
        };

        let map = match resp.json::<Vec<HashMap<String, u32>>>() {
            Ok(m) => m[0].clone(),
            Err(e) => panic!("Something went wrong: {}", e),
        };

        match map.get("wins") {
            Some(w) => w.clone(),
            None => panic!("Something went wrong with the API."),
        }
    }
    /// The total amount of times the team has lost a match.
    ///
    /// This method takes no arguments.
    ///
    /// It returns a `u32` integer.
    pub fn losses(&self) -> u32 {
        let resp = match self
            .client
            .request(&format!("/team/{}/wlt", self.team_number)[..])
        {
            Ok(resp) => resp,
            Err(e) => panic!("Something went wrong: {}", e),
        };

        let map = match resp.json::<Vec<HashMap<String, u32>>>() {
            Ok(m) => m[0].clone(),
            Err(e) => panic!("Something went wrong: {}", e),
        };

        match map.get("losses") {
            Some(l) => l.clone(),
            None => panic!("Something went wrong with the API."),
        }
    }
    /// The amount of times the team has tied a match.
    ///
    /// This method takes no arguments.
    ///
    /// It returns a `u32` integer.
    pub fn ties(&self) -> u32 {
        let resp = match self
            .client
            .request(&format!("/team/{}/wlt", self.team_number)[..])
        {
            Ok(resp) => resp,
            Err(e) => panic!("Something went wrong: {}", e),
        };

        let map = match resp.json::<Vec<HashMap<String, u32>>>() {
            Ok(m) => m[0].clone(),
            Err(e) => panic!("Something went wrong: {}", e),
        };

        match map.get("ties") {
            Some(t) => t.clone(),
            None => panic!("Something went wrong with the API."),
        }
    }
}

#[cfg(test)]
mod tests {
    fn create_client() -> super::Client {
        super::Client::new("1e48fa3b34a8ab86cbec44735c5b6055a141f245455faac878bfa204e35c1a7e")
    }
    #[test]
    fn correct_version() {
        let client = create_client();
        assert_eq!("3.7.0", client.api_version());
    }
    #[test]
    fn check_number() {
        let client = create_client();
        let team = client.team(16405);
        assert_eq!(team.team_number, 16405);
    }
    #[test]
    fn check_compat() {
        let client = create_client();
        let team1 = client.team(16405);
        let team2 = client.team(16405);
        assert_eq!(team1.wins(), team2.wins());
    }
    #[test]
    fn check_numbers() {
        let client = create_client();
        let team1 = client.team(16405);
        let team2 = client.team(16405);
        assert_eq!(team1.team_number, team2.team_number);
    }
}
