use sha2::Sha256;
use sha2::Digest;
use crate::ContentHash;
use crate::Content;

pub fn content_hash(content: &Content) -> ContentHash {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}