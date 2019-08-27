use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Error, Response};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Authentication user model
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
struct AccessToken {
    jwt: String,
}

#[derive(Clone)]
/// HTTP Client structure
pub struct DBClient {
    base_url: String,
    client: reqwest::Client,
    token: AccessToken,
}

impl DBClient {
    /// Creates a new HTTP Client with a base URL
    pub fn new(base_url: String) -> DBClient {
        DBClient {
            base_url: base_url,
            client: reqwest::Client::new(),
            token: AccessToken {
                jwt: "".to_string(),
            },
        }
    }

    /// Get a resource
    pub fn get(&self, path: &str) -> Result<Response, Error> {
        let final_url: String = self.base_url.clone() + path;
        match self.client.get(&final_url).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Post a resource
    pub fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let final_url: String = self.base_url.clone() + path;

        match self
            .client
            .post(&final_url)
            .json(body)
            .headers(headers)
            .send()
        {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Runs a GET request to the base_url to verify that the server is currently available
    /// If the server is available returns true
    pub fn is_db_available(&self) -> bool {
        let response = self.get("/_admin/server/availability");
        match response {
            Ok(r) => r.status().is_success(),
            Err(_) => false,
        }
    }

    /// Runs a POST request to the authentication endpoint
    /// the db client will hold the JWT authentication token, returns true if authentication was
    /// successful
    pub fn authenticate(&mut self, username: &str, password: &str) -> bool {
        let mut authenticated = false;
        let user = User {
            username: username.to_string(),
            password: password.to_string(),
        };
        match self.post("/_open/auth", &user) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let access_token: AccessToken = res.json().unwrap();
                    self.token = access_token;
                    authenticated = true;
                }
            }
            Err(err) => println!("{:?}", err),
        };
        authenticated
    }
}
