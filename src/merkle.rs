use core::fmt;
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::node::Node;

const MAX_BLOCK_LIMIT: usize = 4;

#[derive(Clone, Debug, Default)]
pub struct MerkleTree<T: Display> {
    pub root: Option<Rc<Node<T>>>,
    children: [Rc<Node<T>>; MAX_BLOCK_LIMIT],
}

impl<T> fmt::Display for MerkleTree<T>
where
    T: Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.root.as_ref().unwrap(), f)
    }
}
impl<T> MerkleTree<T>
where
    T: Display + Default,
{
    pub fn new(values: [T; MAX_BLOCK_LIMIT]) -> Self {
        let children = values.map(Node::new_leaf);
        Self {
            root: None,
            children,
        }
    }
    pub fn build_tree(&mut self) {
        let g = MerkleTree::<T>::build_tree_rec(&self.children);
        self.root = g;
    }
    pub fn build_tree_rec(values: &[Rc<Node<T>>]) -> Option<Rc<Node<T>>> {
        let half = values.len() / 2;
        if values.len() == 2 || values.len() == 1 {
            let a = values[0].clone();
            let b = {
                if values.len() == 2 {
                    Some(values[1].clone())
                } else {
                    None
                }
            };
            let y = Node {
                hash: Node::<T>::hash(
                    format!(
                        "{}{}",
                        a.hash,
                        b.as_ref().unwrap_or(&Node::empty_leaf()).hash
                    )
                    .as_str(),
                ),
                left: Some(a),
                right: b,
                value: T::default(),
            };
            return Some(y.into());
        }

        let left = MerkleTree::<T>::build_tree_rec(&values[..half]);
        let right = MerkleTree::<T>::build_tree_rec(&values[half..]);
        let hash = Node::<T>::hash(
            format!(
                "{}{}",
                left.as_ref().unwrap().hash,
                right.as_ref().unwrap().hash
            )
            .as_str(),
        );
        Some(Rc::new(Node {
            value: T::default(),
            left,
            right,
            hash,
        }))
    }
}
