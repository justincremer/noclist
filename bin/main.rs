extern crate noclist;

use noclist::HttpClient;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    client.auth().await;
    client.get_users().await;
}
