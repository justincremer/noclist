extern crate noclist;

use noclist::{HttpClient, HttpResponse};

#[tokio::main]
async fn main() {
    let mut client = HttpClient::new("http://0.0.0.0:8888");
    ensure_success(&client.auth().await);
    let res: HttpResponse = client.get_users().await;
    if check_success(&res) {
        println!("{:?}", res.unwrap());
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
