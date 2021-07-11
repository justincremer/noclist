use std::fmt;

use crate::http::HttpRespone;

#[derive(Debug)]
pub struct Users {
    inner: Vec<String>,
}

impl Users {
    pub fn new(inner: Option<Vec<String>>) -> Self {
        let inner: Vec<String> = match inner {
            Some(i) => i,
            None => Vec::new(),
        };

        Users { inner }
    }

    pub async fn populate_from_response(&mut self, i: HttpRespone) -> Result<(), ()> {
        match i {
            Ok(r) => match r.text().await {
                Ok(text) => {
                    *self = Users::from(text);
                    return Ok(());
                }
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }

        Err(())
    }
}

impl Default for Users {
    fn default() -> Self {
        Users { inner: Vec::new() }
    }
}

impl From<String> for Users {
    fn from(i: String) -> Self {
        let inner = i
            .split("\n")
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>();
        Users { inner }
    }
}

impl fmt::Display for Users {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.inner).unwrap())
    }
}
