extern crate noclist;

use noclist::http::{check_success, HttpClient};
use noclist::users::Users;

use std::process::exit;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    // Initialize empty user vector
    let mut users: Users = Default::default();

    // Authenticate client, attempting 3 times, and smoothly exiting.
    // I would normally write this generically and recursively,
    // but there is no need for a simple appliction such as this.
    let mut success = false;
    for _ in 0..3 {
        match check_success(&client.auth().await) {
            true => {
                success = true;
                break;
            }
            _ => {}
        }
    }

    if !success {
        eprintln!("failed to authenticate client session");
        exit(1);
    };

    success = false;
    // Fetch users, attempting 3 times, and smoothly exiting.
    for _ in 0..3 {
        let res = client.get_users().await;
        // Match over success of users response, populating the users vector on success
        match users.populate_from_response(res).await {
            Ok(_) => {
                success = true;
                break;
            }
            _ => {}
        }
    }

    if success {
        println!("{}", users);
        exit(0);
    };

    exit(1);
}
