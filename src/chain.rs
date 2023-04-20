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
        let mut root_block = Block::new(
            "",
            MerkleTree::new([T::default(), T::default(), T::default(), T::default()]),
        );
        root_block.mine_block();
        Self {
            blocks: vec![root_block],
        }
    }

    pub fn add_new_block(&mut self, block: Block<T>) {}
}
