use std::fmt::Display;

use crate::merkle::MerkleTree;

struct Block<T: Display> {
    prev_block_hash: String,
    nonce: u32,
    data: MerkleTree<T>,
    hash: String,
}

impl<T> Block<T>
where
    T: Display,
{
    pub fn new(prev_block_hash: &str, data: MerkleTree<T>) -> Self {
        let (nonce, hash) = Block::mine_new_block(&data);
        Self {
            prev_block_hash: prev_block_hash.to_string(),
            nonce,
            data,
            hash,
        }
    }

    pub fn mine_new_block(tree: &MerkleTree<T>) -> (u32, String) {
        (2, "".to_string())
    }
}
