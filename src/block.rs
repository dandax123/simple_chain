use sha2::{Digest, Sha256};

use std::fmt::Display;

use crate::merkle::MerkleTree;

const NUM_OF_ZEROS: usize = 3;

pub struct Block<T: std::fmt::Display> {
    pub prev_block_hash: String,
    nonce: u32,
    data: MerkleTree<T>,
    pub hash: String,
}

impl<T> Block<T>
where
    T: Display + Default,
{
    pub fn new(mut data: MerkleTree<T>) -> Self {
        data.build_tree();
        Self {
            prev_block_hash: "".to_string(),
            nonce: 0,
            data,
            hash: "".to_string(),
        }
    }

    pub fn mine_block(&mut self) {
        // println!("Starting mining process");
        let (nonce, hash) = self.mine_new_block(NUM_OF_ZEROS);
        // println!("Finished mining process");
        self.nonce = nonce;
        self.hash = hash;
    }
    pub fn mine_new_block(&mut self, num_of_zeros: usize) -> (u32, String) {
        let mut nonce = 0;
        let mut is_restriction_passed = false;
        let mut hash = String::new();
        while !is_restriction_passed {
            hash = self.hash_block();
            is_restriction_passed = Block::<T>::check_restriction(hash.as_str(), num_of_zeros);
            nonce += 1;
            self.nonce = nonce;
            // println!("Nonce: {}, hash: {} ", nonce, hash);
        }
        (nonce, hash)
    }

    fn check_restriction(hash: &str, num_of_zeros: usize) -> bool {
        let z = "0".to_string();
        hash[..num_of_zeros] == z.repeat(num_of_zeros)
    }

    pub fn hash_block(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.get_string_rep().as_bytes());
        let hash_values: String = format!("{:X}", hasher.finalize());
        hash_values
    }
    pub fn get_string_rep(&self) -> String {
        let a = &self.data.root.as_ref().unwrap().hash;
        format!("{}-{}-{}", self.nonce, a, self.prev_block_hash)
    }
}

impl<T> Display for Block<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Block Hash: {}", self.hash)
    }
}
