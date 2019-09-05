use super::{AccessToken, BaseClient, BaseResponse, Collection, Database, User};

use std::collections::HashMap;

/// HTTP Client structure
pub struct DBClient {
    pub base_url: String,
    pub client: BaseClient,
    // token: AccessToken,
    pub databases: Vec<Database>,
}

impl DBClient {
    /// Creates a new HTTP Client with a base URL
    pub fn new(base_url: &str) -> DBClient {
        DBClient {
            base_url: String::from(base_url),
            client: BaseClient::new(),
            // token: AccessToken {
            //     jwt: "".to_string(),
            // },
            databases: Vec::new(),
        }
    }
    // TODO: helper url concatenation

    /// Runs a GET request to the base_url to verify that the server is currently available
    /// If the server is available returns true
    pub fn is_db_available(&self) -> bool {
        let final_url: String = format!("{}{}", self.base_url, "/_admin/server/availability/");
        match self.client.get(&final_url) {
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
        let final_url: String = format!("{}{}", self.base_url, "/_open/auth/");
        match self.client.post(&final_url, &user) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let access_token: AccessToken = res.json().unwrap();
                    // Create header
                    let header_name: &[u8] = b"Authorization";
                    let header_value: String = format!("Bearer {}", access_token.jwt);
                    // self.token = access_token;
                    authenticated = true;
                    self.client
                        .insert_header(header_name, header_value.as_bytes());
                }
            }
            Err(err) => println!("{:?}", err),
        };
        authenticated
    }

    /// Select a given database for all the next queries.
    /// If the user did not put a / at the beginning it will be inserted
    /// pub fn select_database(&mut self, database_name: &str) {
    ///     self.selected_database = format!(
    ///         "/_db{}",
    ///         if database_name.chars().nth(0).unwrap() == '/' {
    ///             database_name.to_string()
    ///         } else {
    ///             format!("/{}", database_name)
    ///         }
    ///     );
    /// }

    /// Make a list of all the available collections
    pub fn get_all_collections(&self) -> Result<Vec<Collection>, String> {
        let final_url: String = format!("{}{}", self.base_url, "/_api/collection/");
        match self.client.get(&final_url) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: BaseResponse<Vec<Collection>> = res.json().unwrap();
                    return Ok(result.result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    /// Select a specific collection, needs a selected database
    pub fn get_collection(&self, collection_name: &str) -> Result<Collection, String> {
        let final_path = format!(
            "{}{}{}",
            self.base_url, "/_api/collection/", collection_name
        );
        match self.client.get(&final_path) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: Collection = res.json().unwrap();
                    return Ok(result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    /// Create a new collection, needs a selected database
    pub fn post_collection(&self, collection_name: &str) -> Result<Collection, String> {
        let final_path = format!("{}{}", self.base_url, "/_api/collection/");

        // Create a new collection here
        let mut map = HashMap::new();
        map.insert("name", collection_name);

        match self.client.post(&final_path, &map) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: Collection = res.json().unwrap();
                    return Ok(result);
                } else {
                    return Err(res.text().unwrap());
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
}
