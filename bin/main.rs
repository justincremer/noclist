extern crate noclist;
extern crate tokio;

// use noclist::checksum;
use noclist::HttpClient;

#[tokio::main]
async fn main() {
    run().await;

    // let token = "F47D071E3E94B0A6BCCC44B47CD8CA44";
    // let path = "/user";
    // let checksum = checksum(token, path);
}

async fn run() {
    let client = HttpClient::new("http://0.0.0.0:8888");
    client.auth().await;
}
