use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn count(&self) -> usize {
        self.inner.len()
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
