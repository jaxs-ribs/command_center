use crate::Content;
use crate::ContentHash;
use sha2::Digest;
use sha2::Sha256;

pub fn content_hash(content: &Content) -> ContentHash {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
