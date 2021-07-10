extern crate noclist;

use noclist::HttpClient;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    client.auth().await;
    client.get_users().await;

    // let token = "F47D071E3E94B0A6BCCC44B47CD8CA44";
    // let path = "/user";
    // let checksum = checksum(token, path);
}
