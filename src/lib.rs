extern crate reqwest;
extern crate sha2;

use std::borrow::{Borrow, BorrowMut};
use std::convert::TryInto;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response, Url};
use sha2::{Digest, Sha256};

pub struct HttpClient {
    pub client: Client,
    pub base_url: String,
    pub token: String,
}

type HttpResponse = Result<Response, reqwest::Error>;

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
        // let checksum = create_checksum(self.token.clone(), String::from(path));
        let headers = self.construct_headers(&path);
        match self.client.get(url).headers(headers).send().await {
            Ok(r) => println!("{:?}", r),

            Err(e) => eprintln!("failed to fetch users, {}", e),
        };
    }

    fn construct_headers(&self, path: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let checksum = self.create_checksum(path);

        headers.insert(
            HeaderName::from_static("X-Request-Checksum"),
            HeaderValue::from_static(checksum.as_str()),
        );
        headers
    }

    fn create_checksum<'a>(&self, path: &str) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{}{}", self.token, path);

        hasher.update(input);
        hasher
            .finalize()
            .iter()
            .fold(String::new(), |acc, i| acc + i.to_string().as_str())
    }
}

// pub fn bytes_to_str<'a>(i: &'a [u8]) -> Result<&'a str, Utf8Error> {
//     str::from_utf8(i)
// }
