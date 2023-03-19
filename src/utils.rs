use sha2::{Sha256, Digest};
use base16ct;

pub fn create_hash(item: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(item);
    let hash = hasher.finalize();

    let mut buff: Vec<u8> = vec![];
    // maybe convert to Vec<u8>?
    let hex_hash = base16ct::lower::encode_str(&hash, &mut buff);
    // !!!
    let final_hash = hex_hash.unwrap().to_owned();
    final_hash
}
