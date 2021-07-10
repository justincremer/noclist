extern crate reqwest;
extern crate sha2;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Url};
use sha2::{Digest, Sha256};

struct HttpClient {
    client: Client,
    base_url: String,
	checksum: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::new();
		let checksum = String::new();
        HttpClient { client, base_url, checksum }
    }

    pub async fn auth(&self) -> Option<String> {
        let url = match Url::parse(&format!("{}{}", self.base_url, "/auth")) {
			Ok(v) => Some(v),
			_ => {
				eprintln!("failed to parse url");
				None
			}
		};
		
        self.client.get(url.unwrap()).send()?
    }

    pub async fn get_users(&self, base_url: &str, checksum: &'static String) {
        let url = Url::parse(&format!("{}{}", self.base_url, "/users"));
        let headers = construct_headers(&checksum);
    }
}

pub fn checksum<'a>(token: &str, path: &str) -> String {
    let mut hasher = Sha256::new();
    let input = format!("{}{}", token, path);

    hasher.update(input);
    hasher
        .finalize()
        .iter()
        .fold(String::new(), |acc, i| acc + i.to_string().as_str())
}

fn construct_headers(checksum: &'static String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("X_Request_Checksum"),
        HeaderValue::from_static(checksum.as_str()),
    );
    headers
}

// pub fn bytes_to_str<'a>(i: &'a [u8]) -> Result<&'a str, Utf8Error> {
//     str::from_utf8(i)
// }
