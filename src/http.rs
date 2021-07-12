use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::{Client, Url};

use std::error::Error;
use std::str;

use crate::crypt;

pub type HttpRespone = Result<Response, Box<dyn Error>>;

pub fn ensure_success(res: &HttpRespone) {
    match res {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

pub fn check_success(res: &HttpRespone) -> bool {
    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    token: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            client: Client::new(),
            base_url: String::from(base_url),
            token: String::new(),
        }
    }

    pub async fn get_users(&self) -> HttpRespone {
        let path = "/users";
        let url = Url::parse(&format!("{}{}", self.base_url, path)).unwrap();
        let headers = self.construct_headers(&path);
        match self.client.get(url).headers(headers).send().await {
            Ok(r) => Ok(r),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn auth(&mut self) -> HttpRespone {
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
                Ok(r)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn construct_headers(&self, path: &str) -> HeaderMap {
        let checksum = crypt::create_sha256_checksum(self.token.as_str(), path);
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-request-checksum"),
            HeaderValue::from_static(Box::leak(checksum)),
        );
        headers
    }
}
