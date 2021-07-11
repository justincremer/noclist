extern crate noclist;

use serde_json;

use noclist::http::{ensure_success, HttpClient};
use noclist::users::Users;

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    let res = client.auth().await;
    ensure_success(&res);
    let res = client.get_users().await;
    let mut users: Users = Default::default();

    match res {
        Ok(r) => match r.text().await {
            Ok(text) => users = Users::from(text),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }

    println!("{}\nThere are {} users", users, users.count());
}
