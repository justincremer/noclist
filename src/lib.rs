extern crate reqwest;
extern crate sha2;

use sha2::{Digest, Sha256};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Url};

use std::{error, str};

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

        match self
            .client
            .get(url)
            .headers(self.construct_headers(&path))
            .send()
            .await
        {
            Ok(r) => println!("{:?}", r),
            Err(e) => eprintln!("failed to fetch users, {}", e),
        };
    }

    fn construct_headers(&self, path: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let checksum = self.create_checksum::<Sha256>(path);

        headers.insert(
            HeaderName::from_static("x-request-checksum"),
            HeaderValue::from_static(Box::leak(checksum)),
        );
        headers
    }

    fn create_checksum<'a, D: Digest>(&self, path: &str) -> Box<String> {
        let mut hasher = D::new();
        let input = self.token.clone() + path;
        hasher.update(input.as_bytes());
        let mut buf: Vec<u8> = Vec::new();
        buf.copy_from_slice(&hasher.finalize());
        Box::from(String::from_utf8(buf).unwrap())
    }
}
