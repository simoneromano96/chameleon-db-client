use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Error, Response};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// Authentication user model
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone)]
/// Access Token model
struct AccessToken {
    jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Collection model
pub struct Collection {
    id: String,
    name: String,
    status: isize,
    r#type: isize,
    is_system: bool,
    globally_unique_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Common base response body
struct BaseResponseBody<T> {
    error: bool,
    code: isize,
    result: Vec<T>,
}

#[derive(Clone)]
/// HTTP Client structure
pub struct DBClient {
    base_url: String,
    client: reqwest::Client,
    token: AccessToken,
    selected_database: String
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
            selected_database: "".to_string(),
        }
    }

    /// Generate headers for the requests
    fn generate_headers(&self, token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        if !token.is_empty() {
            let bearer: &str = &format!("{} {}", "Bearer", token);
            headers.insert(AUTHORIZATION, (HeaderValue::from_str(bearer)).unwrap());
        }
        headers
    }

    /// Get a resource
    fn get(&self, path: &str) -> Result<Response, Error> {
        let final_url: String = self.base_url.clone() + &self.selected_database + path;
        let headers = self.generate_headers(self.token.jwt.clone());

        match self.client.get(&final_url).headers(headers).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Post a resource
    fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response, Error> {
        let final_url: String = self.base_url.clone() + path;
        let headers = self.generate_headers(self.token.jwt.clone());

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

    /// Make a list of all available databases to whom the user can access
    pub fn get_all_databases(&self) -> Result<Vec<String>, String> {
        match self.get("/_api/database/user") {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: BaseResponseBody<String> = res.json().unwrap();
                    return Ok(result.result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    /// Select a given database for all the next queries.
    /// If the user did not put a / at the beginning it will be inserted
    pub fn select_database(&mut self, database_name: &str) {
        self.selected_database = format!("/_db{}", if database_name.chars().nth(0).unwrap() == '/' { database_name.to_string() } else { format!("/{}", database_name) });
    }

    /// Make a list of all the available collections
    pub fn get_all_collections(&self) -> Result<Vec<Collection>, String> {
        match self.get("/_api/collection") {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: BaseResponseBody<Collection> = res.json().unwrap();
                    return Ok(result.result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => { 
                return Err(err.to_string()) 
            },
        }
    }

    /// Select a specific collection, needs a selected database
    pub fn get_collection(&self, collection_name: &str) -> Result<Collection, String> {
        let final_path = format!("{}/{}", "/_api/collection", collection_name);
        match self.get(&final_path) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: Collection = res.json().unwrap();
                    return Ok(result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => {
                return Err(err.to_string())
            }
        }
    }
}
