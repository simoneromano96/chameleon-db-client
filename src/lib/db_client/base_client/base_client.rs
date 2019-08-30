use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::{Client, Error, Response};
use serde::Serialize;

#[derive(Clone)]
pub struct BaseClient {
    client: Client,
    headers: HeaderMap,
}

impl BaseClient {
    /// Creates a new HTTP Client with a base URL
    pub fn new() -> BaseClient {
        let mut headers = HeaderMap::new();
        // Every request will have a JSON body
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        // Every response must be in JSON
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        
        BaseClient {
            client: reqwest::Client::new(),
            headers: headers,
        }
    }

    /// Add or substitute an header
    pub fn insert_header(&mut self, header_name: &'static str, header_content: &str) {
        self.headers.insert(header_name, HeaderValue::from_str(header_content).unwrap());
    }

    /// Get a resource
    pub fn get(self, path: &str) -> Result<Response, Error> {
        match self.client.get(path).headers(self.headers).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Post a resource
    pub fn post<T: Serialize>(self, path: &str, body: &T) -> Result<Response, Error> {
        match self
            .client
            .post(path)
            .json(body)
            .headers(self.headers)
            .send()
        {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }
}