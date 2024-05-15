/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

const API_URL: &str = "http://localhost:8180/";
const REALM: &str = "WMS";
const CLIENT_ID: &str = "workshop_client";
const CLIENT_SECRET: &str = "Ip7GUqM8mRuIHMcq3tOuuHCaejSwSk3S";

#[derive(Clone)]
pub struct Keycloak {
    pub token: String,
    pub username: String,
    pub groups: Vec<String>,
}

impl Keycloak {
    pub fn new() -> Keycloak {
        Keycloak {
            token: String::new(),
            username: String::new(),
            groups: Vec::new(),
        }
    }

    #[tokio::main]
    pub async fn login_user(username: &str, password: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/realms/{}/protocol/openid-connect/token", API_URL, REALM))
            .form(&[
                ("client_id", CLIENT_ID),
                ("client_secret", CLIENT_SECRET),
                ("username", username),
                ("password", password),
                ("grant_type", "password"),
            ])
            .send()
            .await?;
        match response.error_for_status_ref().err() {
            Some(err) => Err(err),
            None => {
                let token: serde_json::Value = response.json().await?;
                Ok(token["access_token"].as_str().unwrap().to_string())
            }
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_groups(&mut self, groups: Vec<String>) {
        self.groups = groups;
    }

    pub fn clear(&mut self) {
        self.token.clear();
        self.username.clear();
        self.groups.clear();
    }
}