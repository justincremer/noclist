extern crate hex;
extern crate reqwest;
extern crate sha2;

use hex::ToHex;
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

    pub async fn get_users(&self) {
        let path = "/users";
        let url = Url::parse(&format!("{}{}", self.base_url, path)).unwrap();
        let headers = self.construct_headers(&path);

        match self.client.get(url).headers(headers).send().await {
            Ok(r) => println!("{:?}", r),
            Err(e) => eprintln!("failed to fetch users, {}", e),
        };
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
                    .to_string();
            }
            Err(e) => eprintln!("failed to authenticate, {}", e),
        };
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
