extern crate noclist;

use noclist::http::{ensure_success, HttpClient};
use noclist::users::Users;

use std::process::exit;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    let mut users: Users = Default::default();

    // Authenticate client
    ensure_success(&client.auth().await);

    // Fetch users, retrying 3 times, and smoothly exiting
    let mut acc = 0;
    while acc < 3 {
        let res = client.get_users().await;
        // Match over success of users response, populating the users vector on success
        match users.populate_from_response(res).await {
            Ok(_) => {
                println!("{}", users);
                exit(0);
            }
            Err(_) => {}
        }
        acc += 1;
    }
    exit(1);
}
