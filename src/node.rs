use sha2::{Digest, Sha256};
use std::{
    fmt::{self, Debug, Display},
    rc::Rc,
};
#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    pub value: T,
    pub hash: String,
    pub left: Option<Rc<Node<T>>>,
    pub right: Option<Rc<Node<T>>>,
}
impl<T> Node<T>
where
    T: Display + Default,
{
    pub fn new_leaf(value: T) -> Rc<Node<T>> {
        let y = Node {
            hash: Node::<T>::hash(value.to_string().as_ref()),
            value,
            right: None,
            left: None,
        };
        Rc::new(y)
    }
    pub fn empty_leaf() -> Rc<Node<T>> {
        let y = Node {
            value: T::default(),
            right: None,
            left: None,
            hash: Node::<T>::hash(T::default().to_string().as_ref()),
        };
        Rc::new(y)
    }
    pub fn hash(value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        let hash_values: String = format!("{:X}", hasher.finalize());
        hash_values
    }
}

impl<T> fmt::Display for Node<T>
where
    T: Default + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Root: {}", &self.hash)?;
        if let Some(a) = &self.left {
            // write!(f, "Left: ")?;
            std::fmt::Display::fmt(&a, f)?;
        };
        if let Some(b) = &self.right {
            // write!(f, "Right: ")?;
            // writeln!(f)?;
            std::fmt::Display::fmt(&b, f)?;
        };
        Ok(())
    }
}
