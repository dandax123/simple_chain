use std::fmt::Display;

use crate::{block::Block, merkle::MerkleTree};

#[derive(Default)]
pub struct Chain<T: Display> {
    pub blocks: Vec<Block<T>>,
}

impl<T> Chain<T>
where
    T: Display + Default,
{
    pub fn new() -> Self {
        let mut root_block = Block::new(MerkleTree::new([
            T::default(),
            T::default(),
            T::default(),
            T::default(),
        ]));
        root_block.mine_block();
        Self {
            blocks: vec![root_block],
        }
    }

    pub fn add_new_block(&mut self, mut new_block: Block<T>) {
        let y = self.blocks.as_slice();
        let b: &Block<T> = &y[y.len() - 1];
        let prev_hash = &b.hash;
        new_block.prev_block_hash = prev_hash.to_string();

        new_block.mine_block();

        self.blocks.push(new_block);
    }
    pub fn view_chain(&self) {
        self.blocks
            .iter()
            .enumerate()
            .for_each(|b| println!("Block: {} {}", b.0, b.1));
    }
}
