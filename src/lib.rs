extern crate hex;
extern crate reqwest;
extern crate sha2;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response, Url};
use sha2::{Digest, Sha256};
use std::{error, str};

pub type HttpResponse = Result<Response, Box<dyn error::Error>>;

pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    pub token: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: String::from(base_url),
            token: String::new(),
        }
    }

    pub async fn get_users(&self) -> Result<Response, Box<dyn error::Error>> {
        let path = "/users";
        let url = Url::parse(&format!("{}{}", self.base_url, path)).unwrap();
        let headers = self.construct_headers(&path);
        let res = self.client.get(url).headers(headers).send().await?;
        Ok(res)
    }

    pub async fn auth(&mut self) -> Result<Response, Box<dyn error::Error>> {
        let url = Url::parse(&format!("{}{}", self.base_url, "/auth")).unwrap();
        let res = self.client.get(url).send().await?;
        self.token = res
            .headers()
            .get("badsec-authentication-token")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Ok(res)
    }

    fn construct_headers(&self, path: &str) -> HeaderMap {
        let checksum = self.create_sha256_checksum(path);
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-request-checksum"),
            HeaderValue::from_static(Box::leak(checksum)),
        );
        headers
    }

    fn create_sha256_checksum<'a>(&self, path: &str) -> Box<String> {
        let mut hasher = Sha256::new();
        let input = self.token.to_owned() + path;
        let mut buf: [u8; 32] = [0; 32];

        hasher.update(input);
        buf.copy_from_slice(&hasher.finalize());

        Box::from(hex::encode(&buf))
    }
}
