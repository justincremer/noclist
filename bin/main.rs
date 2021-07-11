extern crate noclist;

use std::ascii::AsciiExt;

use hex;
use noclist::{HttpClient, HttpResponse};

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    ensure_success(&client.auth().await);
    let res: HttpResponse = client.get_users().await;
    match res {
        Ok(r) => match r.text().await {
            Ok(text) => println!("{}", text),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

fn ensure_success(res: &HttpResponse) {
    match res {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

fn check_success(res: &HttpResponse) -> bool {
    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}
