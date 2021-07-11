extern crate noclist;

use noclist::http::{ensure_success, HttpClient};
use noclist::users::Users;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    let mut users: Users = Default::default();

    // Authenticate client
    ensure_success(&client.auth().await);

    // Match over success of users response, populating the users vector on success
    match client.get_users().await {
        Ok(r) => match r.text().await {
            Ok(text) => users = Users::from(text),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }

    println!("{}\nThere are {} users", users, users.count());
}
