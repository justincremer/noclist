extern crate reqwest;
extern crate sha2;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Url};
use sha2::{Digest, Sha256};
use std::str;

pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    pub token: String,
}

// type HttpResponse = Result<Response, reqwest::Error>;

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: String::from(base_url),
            token: String::new(),
        }
    }

    pub async fn auth(&mut self) {
        let url = Url::parse(&format!("{}{}", self.base_url, "/auth")).unwrap();
        match self.client.get(url).send().await {
            Ok(r) => {
                self.token = r
                    .headers()
                    .get("badsec-authentication-token")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("-", "")
                    .to_string();
            }
            Err(e) => eprintln!("failed to authenticate, {}", e),
        };
    }

    pub async fn get_users(&self) {
        let path = "/users";
        let url = Url::parse(&format!("{}{}", self.base_url, path)).unwrap();
        let headers = self.construct_headers(&path);

        match self.client.get(url).headers(headers.unwrap()).send().await {
            Ok(r) => println!("{:?}", r),
            Err(e) => eprintln!("failed to fetch users, {}", e),
        };
    }

    fn construct_headers(&self, path: &str) -> Result<HeaderMap, &str> {
        match self.create_sha256_checksum(path) {
            Ok(v) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("x-request-checksum"),
                    HeaderValue::from_static(Box::leak(v)),
                );
                Ok(headers)
            }
            Err(e) => Err(e),
        }
    }

    fn create_sha256_checksum<'a>(&self, path: &str) -> Result<Box<String>, &str> {
        let mut hasher = Sha256::new();
        let input = self.token.clone() + path;
        let mut buf: [u8; 32] = [0; 32];
        hasher.update(input.as_bytes());
        buf.copy_from_slice(&hasher.finalize());
        match str::from_utf8(&buf) {
            Ok(s) => Ok(Box::from(s.to_string())),
            Err(e) => Err(format!("failed to parse buffer to utf8, {}", e)),
        }
    }
}
