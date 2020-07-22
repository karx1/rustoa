//! # RusTOA
//!
//! `rustoa` is a crate you can use to access The Orange Alliance API.
//! This crate makes it easy to access the official First Tech Challenge API
//! and use it in your Rust projects.

use reqwest::blocking::Response;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

/// The main RusTOA client.
///
/// You can use the [Client](struct.Client.html) to get the API version
/// and create a [Team](struct.Team.html) object.
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
    /// It returns a [Client](struct.Client.html) object.
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
    /// This method is used to get an instance of [`Team`](struct.Team.html).
    /// # Arguments
    ///
    /// * `team_number` - The FTC team number as a `u32` integer.
    ///
    /// It returns a [Team](struct.Team.html) object with the necessary data.
    pub fn team(&self, team_number: u32) -> Team {
        Team::new(team_number, self.clone())
    }
}

/// A struct used to access an FTC team.
///
/// Do not create this struct yourself. Instead use your [`Client`](struct.Client.html) instance.
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

    /// Basic information of the team.
    ///
    /// This method takes no arguments.
    ///
    /// It returns a `HashMap<String, String>`.
    ///
    /// # Panics
    ///
    /// This method can panic in the following ways:
    /// - The HTTP request was not successful
    /// - The data received from the API was invalid JSON
    /// - The data received was in the wrong format
    pub fn properties(&self) -> HashMap<String, String, RandomState> {
        let resp = match self.client.request(&format!("/team/{}/", self.team_number)[..]) {
            Ok(resp) => resp,
            Err(e) => panic!("Something went wrong: {}", e)
        };

        let map: serde_json::Value = match serde_json::from_str(&*match resp.text() {
            Ok(text) => text,
            Err(e) => panic!("Something went wrong: {}", e)
        }) {
            Ok(m) => m,
            Err(e) => panic!("Something went wrong: {}", e)
        };

        let item = match map.as_array() {
            Some(n) => n,
            None => panic!("Something went wrong")
        };

        let value = item[0].clone();

        let new = match value.as_object() {
            Some(m) => m,
            None => panic!("Something went wrong")
        };

        let mut new_map: HashMap<String, String> = HashMap::new();

        for x in new.iter() {
            let key = x.0.clone();
            let value = match x.1 {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => match n.as_u64() {
                    Some(u) => u.to_string(),
                    None => panic!("Something went wrong")
                }
                serde_json::Value::Null => "null".to_string(),
                _ => panic!("Something went wrong")
            };
            new_map.insert(key, value);
        }

        new_map
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
        let year1 = match team1.properties().get("rookie_year") {
            Some(y) => y.clone(),
            None => panic!("Somethign went wrong")
        };
        let year2 = match team2.properties().get("rookie_year") {
            Some(y) => y.clone(),
            None => panic!("Something went wrong")
        };
        assert_eq!(year1, year2);
    }
    #[test]
    fn check_numbers() {
        let client = create_client();
        let team1 = client.team(16405);
        let team2 = client.team(16405);
        assert_eq!(team1.team_number, team2.team_number);
    }
    #[test]
    fn test_property() {
        let client = create_client();
        let team = client.team(16405);
        let year = match team.properties().get("rookie_year") {
            Some(y) => y.clone(),
            None => panic!("Something went wrong")
        };
        assert_eq!("2019", year);
    }
}
