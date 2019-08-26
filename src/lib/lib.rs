use reqwest;
use std;

#[allow(dead_code)]
#[derive(Clone)]
/// HTTP Client structure
pub struct DBClient {
    base_url: String,
    client: reqwest::Client,
}

impl DBClient {
    // A public constructor method
    /// Creates a new HTTP Client with a base URL
    pub fn new(base_url: String) -> DBClient {
        DBClient {
            base_url: base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Get a resource
    pub fn get(&self, path: &str) {
        let final_url: String = self.base_url.clone() + path;

        let mut res  = self.client.get(&final_url).send().unwrap();
        println!("Status: {}", res.status());
        println!("Headers:\n{:?}", res.headers());

        // copy the response body directly to stdout
        std::io::copy(&mut res, &mut std::io::stdout());
    }
}
