use sha2::{Digest, Sha256};

pub fn create_sha256_checksum(token: &str, path: &str) -> Box<String> {
    let mut hasher = Sha256::new();
    let input = format!("{}{}", token, path);
    let mut buf: [u8; 32] = [0; 32];

    hasher.update(input);
    buf.copy_from_slice(&hasher.finalize());

    Box::from(hex::encode(&buf))
}
