use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
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
        // ? Are there any other possible headers ?
        BaseClient {
            client: reqwest::Client::new(),
            headers: headers,
        }
    }

    /// Add or substitute an header
    pub fn insert_header(&mut self, header_name: &[u8], header_content: &[u8]) {
        self.headers.insert(
            HeaderName::from_bytes(header_name).unwrap(),
            HeaderValue::from_bytes(header_content).unwrap(),
        );
    }

    /// Get a resource
    pub fn get(&self, path: &str) -> Result<Response, Error> {
        let headers = self.headers.clone();
        match self.client.get(path).headers(headers).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Post a resource
    pub fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<Response, Error> {
        let headers = self.headers.clone();
        match self.client.post(path).json(body).headers(headers).send() {
            Ok(response) => Ok(response),
            Err(error) => Err(error),
        }
    }

    /// Delete a resource
    pub fn delete(&self, path: &str) -> Result<Response, Error> {
        let headers = self.headers.clone();
        match self.client.delete(path).headers(headers).send() {
            Ok(response) => Ok(response), 
            Err(error) => Err(error)
        }
    }
}
