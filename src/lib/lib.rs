use reqwest;

#[allow(dead_code)]
#[derive(Clone)]
/// HTTP Client structure
pub struct DBClient {
    base_url: String,
    client: reqwest::Client,
}

impl DBClient {
    /// Creates a new HTTP Client with a base URL
    pub fn new(base_url: String) -> DBClient {
        DBClient {
            base_url: base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Runs a GET request to the base_url to verify that the server is currently available
    /// If the server is available returns true
    pub fn db_available(&self) -> bool {
        let response = self.get("/_admin/server/availability");
        match response {
            Ok(r) => r.status().is_success(),
            Err(_) => false
        }
    }

    /// Get a resource
    pub fn get(&self, path: &str) -> Result<reqwest::Response, reqwest::Error> {
        let final_url: String = self.base_url.clone() + path;
        match self.client.get(&final_url).send()
        {
            Ok(response) => Ok(response),
            Err(error) => Err(error)
        }
    }
}
