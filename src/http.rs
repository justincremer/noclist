use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Url};

use crate::crypt;
use std::str;

use reqwest::Response;
use std::error;

pub type HttpRespone = Result<Response, Box<dyn error::Error>>;

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

    pub async fn get_users(&self) -> HttpRespone {
        let path = "/users";
        let url = Url::parse(&format!("{}{}", self.base_url, path)).unwrap();
        let headers = self.construct_headers(&path);
        let res = self.client.get(url).headers(headers).send().await?;

        Ok(res)
    }

    pub async fn auth(&mut self) -> HttpRespone {
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
        let checksum = crypt::create_sha256_checksum(self.token.as_str(), path);
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-request-checksum"),
            HeaderValue::from_static(Box::leak(checksum)),
        );
        headers
    }
}
